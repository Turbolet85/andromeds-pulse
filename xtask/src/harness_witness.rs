//! The exit witness's reader — what `scripts/exit-witness.c` recorded of the
//! app process's end, reduced to one closed label for the settle verdict.
//!
//! `agent-run.sh boot` loads the library into the app process alone; it
//! appends closed-shape JSON lines to `logs/exit-witness.jsonl` under the
//! data dir. This module reads that file under a bound and lets nothing of a
//! line out: the label is all the settle verdict carries. Harness-written;
//! the product never reads the file.

use std::path::Path;

use serde_json::Value;

pub(crate) const WITNESS_FILE: &str = "exit-witness.jsonl";

const MAX_LINES: usize = 64;
/// The library writes each line with one `write` of at most this many bytes.
const MAX_LINE_BYTES: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Loaded,
    End,
    RuntimeExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WitnessLine {
    kind: LineKind,
    pid: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum WitnessRead {
    Unset,
    Unreadable,
    Lines(Vec<WitnessLine>),
}

/// The settle verdict's `exit_witness` member for the app `pid`, whose exit
/// record (`exit N` / `signal N (NAME)`) is `ended`.
pub(crate) fn label(
    data_dir: Option<&Path>,
    pid: Option<u32>,
    ended: Option<&str>,
) -> &'static str {
    match data_dir {
        Some(dir) => decide(
            pid,
            ended,
            &read_witness(&dir.join("logs").join(WITNESS_FILE)),
        ),
        None => "unset",
    }
}

/// The witness core — pure so the arms are pinnable without a process. Only
/// the lines of `pid` count: a forked copy of the app writes under its own.
/// A file that holds no `loaded` line for `pid` cannot be read for it.
fn decide(pid: Option<u32>, ended: Option<&str>, read: &WitnessRead) -> &'static str {
    let lines = match read {
        WitnessRead::Unset => return "unset",
        WitnessRead::Unreadable => return "unreadable",
        WitnessRead::Lines(lines) => lines,
    };
    let Some(pid) = pid else {
        return "unreadable";
    };
    let wrote = |kind| {
        lines
            .iter()
            .any(|line| line.pid == pid && line.kind == kind)
    };
    if wrote(LineKind::End) {
        "exit-call"
    } else if wrote(LineKind::RuntimeExit) {
        "runtime-exit"
    } else if !wrote(LineKind::Loaded) {
        "unreadable"
    } else if ended.is_some_and(|record| record.starts_with("exit ")) {
        // The process ended with a code by a path the library cannot see.
        "no-record"
    } else {
        "loaded"
    }
}

fn parse_line(line: &[u8]) -> Option<WitnessLine> {
    if line.len() > MAX_LINE_BYTES || !line.iter().all(|byte| (0x20..=0x7e).contains(byte)) {
        return None;
    }
    let record = serde_json::from_slice::<Value>(line).ok()?;
    let record = record.as_object()?;
    let kind = match record.get("kind")?.as_str()? {
        "loaded" => LineKind::Loaded,
        "end" => LineKind::End,
        "runtime-exit" => LineKind::RuntimeExit,
        _ => return None,
    };
    let pid = u32::try_from(record.get("pid")?.as_u64()?).ok()?;
    Some(WitnessLine { kind, pid })
}

/// Every line or none: one line outside the grammar makes the file
/// unreadable, never a partial record.
fn parse_lines(bytes: &[u8]) -> Option<Vec<WitnessLine>> {
    let body = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if body.is_empty() {
        return Some(Vec::new());
    }
    let lines: Vec<&[u8]> = body.split(|byte| *byte == b'\n').collect();
    if lines.len() > MAX_LINES {
        return None;
    }
    lines.into_iter().map(parse_line).collect()
}

fn read_witness(path: &Path) -> WitnessRead {
    let size = match std::fs::metadata(path) {
        Ok(meta) if meta.is_file() => meta.len(),
        Ok(_) => return WitnessRead::Unreadable,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return WitnessRead::Unset,
        Err(_) => return WitnessRead::Unreadable,
    };
    if size > (MAX_LINES * (MAX_LINE_BYTES + 1)) as u64 {
        return WitnessRead::Unreadable;
    }
    match std::fs::read(path)
        .ok()
        .and_then(|bytes| parse_lines(&bytes))
    {
        Some(lines) => WitnessRead::Lines(lines),
        None => WitnessRead::Unreadable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PID: u32 = 374_465;

    /// The dev-host control's main-thread record, whole: the display server
    /// stopped two seconds after ready, read under the research probe.
    const CONTROL_END_LINE: &str = r#"{"kind":"end","pid":374465,"tid":374465,"comm":"pulse-app","call":"_exit","code":1,"errno":11,"frames":[{"m":"exit_witness.so","s":"_exit","o":"0x16a1"},{"m":"libgdk-3.so.0","s":"","o":"0x8ca2d"},{"m":"libX11.so.6","s":"_XIOError","o":"0x3fa4c"},{"m":"libX11.so.6","s":"_XEventsQueued","o":"0x44fbf"},{"m":"libX11.so.6","s":"XPending","o":"0x34208"},{"m":"libgdk-3.so.0","s":"","o":"0x812be"},{"m":"libglib-2.0.so.0","s":"","o":"0x637e2"},{"m":"libglib-2.0.so.0","s":"","o":"0x63cb3"},{"m":"libglib-2.0.so.0","s":"g_main_context_iteration","o":"0x64055"},{"m":"libgtk-3.so.0","s":"gtk_main_iteration_do","o":"0x1ead7f"},{"m":"pulse-app","s":"","o":"0x2a53c36"},{"m":"pulse-app","s":"","o":"0x2d308ad"},{"m":"pulse-app","s":"","o":"0x29654a3"},{"m":"pulse-app","s":"","o":"0x2965119"},{"m":"pulse-app","s":"","o":"0x3041414"},{"m":"pulse-app","s":"","o":"0x2db0c25"},{"m":"libc.so.6","s":"","o":"0x27781"},{"m":"libc.so.6","s":"__libc_start_main","o":"0x278b9"},{"m":"pulse-app","s":"","o":"0x18cafa5"}]}"#;

    fn loaded(pid: u32) -> String {
        format!(r#"{{"kind":"loaded","pid":{pid},"comm":"pulse-app"}}"#)
    }

    fn runtime_exit(pid: u32) -> String {
        format!(r#"{{"kind":"runtime-exit","pid":{pid}}}"#)
    }

    fn end(pid: u32) -> String {
        format!(
            r#"{{"kind":"end","pid":{pid},"tid":{pid},"comm":"pulse-app","call":"exit","code":1,"errno":0,"frames":[]}}"#
        )
    }

    fn read_of(lines: &[String]) -> WitnessRead {
        let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
        match parse_lines(text.as_bytes()) {
            Some(lines) => WitnessRead::Lines(lines),
            None => WitnessRead::Unreadable,
        }
    }

    #[test]
    fn no_witness_file_is_unset() {
        assert_eq!(
            decide(Some(PID), Some("exit 1"), &WitnessRead::Unset),
            "unset"
        );
        assert_eq!(label(None, Some(PID), None), "unset");
    }

    #[test]
    fn an_unreadable_file_is_unreadable_whatever_the_exit_record_says() {
        assert_eq!(
            decide(Some(PID), None, &WitnessRead::Unreadable),
            "unreadable"
        );
        assert_eq!(
            decide(Some(PID), Some("exit 1"), &WitnessRead::Unreadable),
            "unreadable"
        );
    }

    #[test]
    fn a_loaded_line_alone_under_a_live_or_signalled_app_is_loaded() {
        let read = read_of(&[loaded(PID)]);
        assert_eq!(decide(Some(PID), None, &read), "loaded");
        assert_eq!(decide(Some(PID), Some("signal 15 (TERM)"), &read), "loaded");
    }

    #[test]
    fn an_end_line_is_exit_call_with_or_without_the_runtime_line() {
        let direct = read_of(&[loaded(PID), end(PID)]);
        assert_eq!(decide(Some(PID), Some("exit 1"), &direct), "exit-call");
        let through_the_linker = read_of(&[loaded(PID), end(PID), runtime_exit(PID)]);
        assert_eq!(
            decide(Some(PID), Some("exit 1"), &through_the_linker),
            "exit-call"
        );
    }

    #[test]
    fn the_dev_host_control_line_reads_exit_call() {
        let read = read_of(&[loaded(PID), CONTROL_END_LINE.to_string()]);
        assert_eq!(
            read,
            WitnessRead::Lines(vec![
                WitnessLine {
                    kind: LineKind::Loaded,
                    pid: PID
                },
                WitnessLine {
                    kind: LineKind::End,
                    pid: PID
                },
            ])
        );
        assert_eq!(decide(Some(PID), Some("exit 1"), &read), "exit-call");
    }

    #[test]
    fn a_runtime_exit_line_without_an_end_line_is_runtime_exit() {
        let read = read_of(&[loaded(PID), runtime_exit(PID)]);
        assert_eq!(decide(Some(PID), Some("exit 1"), &read), "runtime-exit");
    }

    #[test]
    fn a_loaded_line_alone_under_an_exit_code_is_no_record() {
        let read = read_of(&[loaded(PID)]);
        assert_eq!(decide(Some(PID), Some("exit 1"), &read), "no-record");
        assert_eq!(decide(Some(PID), Some("exit 0"), &read), "no-record");
    }

    #[test]
    fn only_the_lines_of_the_app_pid_count() {
        let other = PID + 1;
        let read = read_of(&[loaded(PID), loaded(other), end(other), runtime_exit(other)]);
        assert_eq!(decide(Some(PID), None, &read), "loaded");
        assert_eq!(decide(Some(PID), Some("exit 1"), &read), "no-record");
    }

    #[test]
    fn a_file_with_no_loaded_line_for_the_pid_is_unreadable() {
        let read = read_of(&[loaded(PID + 1)]);
        assert_eq!(decide(Some(PID), Some("exit 1"), &read), "unreadable");
        assert_eq!(
            decide(Some(PID), None, &WitnessRead::Lines(Vec::new())),
            "unreadable"
        );
        assert_eq!(decide(None, None, &read_of(&[loaded(PID)])), "unreadable");
    }

    #[test]
    fn the_reader_takes_the_three_kinds_and_nothing_else() {
        assert_eq!(
            read_of(&[loaded(PID), end(PID), runtime_exit(PID)]),
            WitnessRead::Lines(vec![
                WitnessLine {
                    kind: LineKind::Loaded,
                    pid: PID
                },
                WitnessLine {
                    kind: LineKind::End,
                    pid: PID
                },
                WitnessLine {
                    kind: LineKind::RuntimeExit,
                    pid: PID
                },
            ])
        );
        for bad in [
            r#"{"kind":"child","pid":7}"#,
            r#"{"kind":"loaded"}"#,
            r#"{"kind":"loaded","pid":"7"}"#,
            r#"{"kind":"loaded","pid":-7}"#,
            r#"{"kind":"loaded","pid":4294967296}"#,
            r#"{"pid":7}"#,
            r#"["loaded",7]"#,
            "not json",
            "",
        ] {
            assert_eq!(
                read_of(&[loaded(PID), bad.to_string()]),
                WitnessRead::Unreadable,
                "{bad}"
            );
        }
    }

    #[test]
    fn the_reader_refuses_a_byte_outside_printable_ascii() {
        for bad in [
            "{\"kind\":\"loaded\",\"pid\":7,\"comm\":\"a\tb\"}",
            "{\"kind\":\"loaded\",\"pid\":7,\"comm\":\"caf\u{e9}\"}",
            "{\"kind\":\"loaded\",\"pid\":7}\r",
        ] {
            assert_eq!(parse_lines(format!("{bad}\n").as_bytes()), None, "{bad:?}");
        }
    }

    #[test]
    fn the_reader_is_bounded_at_64_lines_of_4096_bytes() {
        let at_the_line_bound: Vec<String> = (0..MAX_LINES).map(|_| loaded(PID)).collect();
        assert!(
            matches!(read_of(&at_the_line_bound), WitnessRead::Lines(lines) if lines.len() == 64)
        );
        let past_the_line_bound: Vec<String> = (0..=MAX_LINES).map(|_| loaded(PID)).collect();
        assert_eq!(read_of(&past_the_line_bound), WitnessRead::Unreadable);

        let padded = |total: usize| {
            let head = r#"{"kind":"loaded","pid":7,"comm":""#;
            let tail = r#""}"#;
            format!(
                "{head}{}{tail}",
                "a".repeat(total - head.len() - tail.len())
            )
        };
        assert_eq!(padded(MAX_LINE_BYTES).len(), 4096);
        assert!(matches!(
            read_of(&[padded(MAX_LINE_BYTES)]),
            WitnessRead::Lines(_)
        ));
        assert_eq!(
            read_of(&[padded(MAX_LINE_BYTES + 1)]),
            WitnessRead::Unreadable
        );
    }

    #[test]
    fn the_reader_takes_a_last_line_without_its_newline_and_refuses_a_blank_one() {
        assert_eq!(
            parse_lines(loaded(PID).as_bytes()),
            Some(vec![WitnessLine {
                kind: LineKind::Loaded,
                pid: PID
            }])
        );
        assert_eq!(parse_lines(b""), Some(Vec::new()));
        assert_eq!(
            parse_lines(format!("{}\n\n{}\n", loaded(PID), end(PID)).as_bytes()),
            None
        );
    }

    #[test]
    fn the_file_read_tells_absent_from_present_and_bounds_its_size() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let logs = dir.path().join("logs");
        std::fs::create_dir_all(&logs).expect("logs");
        let path = logs.join(WITNESS_FILE);
        assert_eq!(read_witness(&path), WitnessRead::Unset);
        assert_eq!(label(Some(dir.path()), Some(PID), None), "unset");

        std::fs::write(&path, format!("{}\n", loaded(PID))).expect("write");
        assert_eq!(
            read_witness(&path),
            WitnessRead::Lines(vec![WitnessLine {
                kind: LineKind::Loaded,
                pid: PID
            }])
        );
        assert_eq!(label(Some(dir.path()), Some(PID), None), "loaded");
        assert_eq!(
            label(Some(dir.path()), Some(PID), Some("exit 1")),
            "no-record"
        );

        std::fs::write(&path, vec![b'a'; MAX_LINES * (MAX_LINE_BYTES + 1) + 1]).expect("write");
        assert_eq!(read_witness(&path), WitnessRead::Unreadable);
        // A directory in the file's place is present and not readable.
        std::fs::remove_file(&path).expect("remove");
        std::fs::create_dir(&path).expect("dir");
        assert_eq!(read_witness(&path), WitnessRead::Unreadable);
    }

    /// Controls on the library as built: each way a process ends with a code
    /// leaves its own line pattern, and the library stays out of a child.
    #[cfg(target_os = "linux")]
    mod built_library {
        use super::*;
        use std::path::PathBuf;
        use std::process::{Command, Output};

        const FILE_VAR: &str = "ANDROMEDA_PULSE_EXIT_WITNESS_FILE";

        const EXIT_THROUGH_THE_LINKER: &str = "#include <stdlib.h>\nint main(void){ exit(7); }\n";
        const DIRECT_UNDERSCORE_EXIT: &str = "#include <unistd.h>\nint main(void){ _exit(7); }\n";
        const MAIN_RETURNS: &str = "int main(void){ return 7; }\n";
        const DIRECT_SYSTEM_CALL: &str = "#define _GNU_SOURCE\n#include <unistd.h>\n#include <sys/syscall.h>\nint main(void){ syscall(SYS_exit_group, 7); return 0; }\n";
        /// Starts `argv[1]` and waits for it; with `argv[2]` and `argv[3]`
        /// it first sets the preload and the witness file again.
        const PARENT: &str = "#include <stdlib.h>\n#include <unistd.h>\n#include <sys/wait.h>\nint main(int c, char **v){ if (c > 3) { setenv(\"LD_PRELOAD\", v[2], 1); setenv(\"ANDROMEDA_PULSE_EXIT_WITNESS_FILE\", v[3], 1); } pid_t p = fork(); if (p == 0) { execl(v[1], v[1], (char*)0); _exit(99); } int st; waitpid(p, &st, 0); return 0; }\n";
        const ENV_REPORT: &str = "#include <stdio.h>\n#include <stdlib.h>\nint main(void){ printf(\"LD_PRELOAD=%s FILE=%s\\n\", getenv(\"LD_PRELOAD\") ? \"set\" : \"unset\", getenv(\"ANDROMEDA_PULSE_EXIT_WITNESS_FILE\") ? \"set\" : \"unset\"); return 0; }\n";

        struct Bench {
            dir: tempfile::TempDir,
            library: PathBuf,
            witness: PathBuf,
        }

        // A missing compiler fails the control; it never skips it.
        fn cc(args: &[&std::ffi::OsStr]) {
            let output = Command::new("cc")
                .args(args)
                .output()
                .expect("`cc` MUST be on PATH: the controls build the library");
            assert!(
                output.status.success(),
                "cc failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        fn bench() -> Bench {
            let dir = tempfile::TempDir::new().expect("tmp");
            let library = dir.path().join("exit-witness.so");
            let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("workspace root")
                .join("scripts")
                .join("exit-witness.c");
            cc(&[
                "-shared".as_ref(),
                "-fPIC".as_ref(),
                "-O2".as_ref(),
                "-o".as_ref(),
                library.as_os_str(),
                source.as_os_str(),
            ]);
            let witness = dir.path().join(WITNESS_FILE);
            Bench {
                dir,
                library,
                witness,
            }
        }

        impl Bench {
            fn child(&self, name: &str, source: &str) -> PathBuf {
                let c_file = self.dir.path().join(format!("{name}.c"));
                std::fs::write(&c_file, source).expect("write child source");
                let binary = self.dir.path().join(name);
                cc(&[
                    "-O0".as_ref(),
                    "-o".as_ref(),
                    binary.as_os_str(),
                    c_file.as_os_str(),
                ]);
                binary
            }

            fn run(&self, binary: &Path, args: &[&std::ffi::OsStr]) -> (u32, Output) {
                let child = Command::new(binary)
                    .args(args)
                    .env("LD_PRELOAD", &self.library)
                    .env(FILE_VAR, &self.witness)
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .expect("spawn child");
                let pid = child.id();
                (pid, child.wait_with_output().expect("wait child"))
            }

            fn records(&self) -> Vec<Value> {
                std::fs::read_to_string(&self.witness)
                    .expect("witness file")
                    .lines()
                    .map(|line| serde_json::from_str(line).expect("a JSON line"))
                    .collect()
            }

            fn kinds_of(&self, pid: u32) -> Vec<String> {
                self.records()
                    .iter()
                    .filter(|record| record["pid"] == pid)
                    .map(|record| record["kind"].as_str().expect("kind").to_string())
                    .collect()
            }

            fn label_of(&self, pid: u32, ended: &str) -> &'static str {
                decide(Some(pid), Some(ended), &read_witness(&self.witness))
            }
        }

        #[test]
        fn exit_through_the_linker_leaves_end_then_runtime_exit_and_reads_exit_call() {
            let bench = bench();
            let (pid, output) = bench.run(&bench.child("k_exit", EXIT_THROUGH_THE_LINKER), &[]);
            assert_eq!(output.status.code(), Some(7));
            assert_eq!(bench.kinds_of(pid), ["loaded", "end", "runtime-exit"]);
            let end = &bench.records()[1];
            assert_eq!(end["call"], "exit");
            assert_eq!(end["code"], 7);
            assert_eq!(end["comm"], "k_exit");
            assert_eq!(end["frames"][0]["m"], "exit-witness.so");
            assert_eq!(end["frames"][0]["s"], "exit");
            assert_eq!(end["frames"][1]["m"], "k_exit");
            assert_eq!(bench.label_of(pid, "exit 7"), "exit-call");
        }

        #[test]
        fn a_direct_underscore_exit_leaves_end_alone_and_reads_exit_call() {
            let bench = bench();
            let (pid, output) = bench.run(&bench.child("k__exit", DIRECT_UNDERSCORE_EXIT), &[]);
            assert_eq!(output.status.code(), Some(7));
            assert_eq!(bench.kinds_of(pid), ["loaded", "end"]);
            let end = &bench.records()[1];
            assert_eq!(end["call"], "_exit");
            assert_eq!(end["code"], 7);
            assert_eq!(bench.label_of(pid, "exit 7"), "exit-call");
        }

        #[test]
        fn main_returning_leaves_runtime_exit_alone_and_reads_runtime_exit() {
            let bench = bench();
            let (pid, output) = bench.run(&bench.child("k_return", MAIN_RETURNS), &[]);
            assert_eq!(output.status.code(), Some(7));
            assert_eq!(bench.kinds_of(pid), ["loaded", "runtime-exit"]);
            assert_eq!(bench.label_of(pid, "exit 7"), "runtime-exit");
        }

        #[test]
        fn a_direct_system_call_leaves_loaded_alone_and_reads_no_record() {
            let bench = bench();
            let (pid, output) = bench.run(&bench.child("k_syscall", DIRECT_SYSTEM_CALL), &[]);
            assert_eq!(output.status.code(), Some(7));
            assert_eq!(bench.kinds_of(pid), ["loaded"]);
            assert_eq!(bench.label_of(pid, "exit 7"), "no-record");
        }

        fn loaded_lines(bench: &Bench) -> usize {
            bench
                .records()
                .iter()
                .filter(|record| record["kind"] == "loaded")
                .count()
        }

        #[test]
        fn a_child_of_the_witnessed_process_does_not_load_the_library() {
            let bench = bench();
            let report = bench.child("k_env", ENV_REPORT);
            let (pid, output) = bench.run(&bench.child("k_parent", PARENT), &[report.as_os_str()]);
            assert_eq!(output.status.code(), Some(0));
            assert_eq!(
                String::from_utf8_lossy(&output.stdout).trim(),
                "LD_PRELOAD=unset FILE=unset"
            );
            assert_eq!(loaded_lines(&bench), 1);
            assert_eq!(bench.kinds_of(pid), ["loaded", "runtime-exit"]);
            assert_eq!(bench.records().len(), 2, "the child wrote no line");
        }

        // The known positive of the control above: the same pair, the child
        // started with the preload set again, is seen loading the library.
        #[test]
        fn a_child_started_with_the_preload_set_again_loads_the_library() {
            let bench = bench();
            let report = bench.child("k_env", ENV_REPORT);
            let (pid, output) = bench.run(
                &bench.child("k_parent", PARENT),
                &[
                    report.as_os_str(),
                    bench.library.as_os_str(),
                    bench.witness.as_os_str(),
                ],
            );
            assert_eq!(output.status.code(), Some(0));
            assert_eq!(loaded_lines(&bench), 2);
            assert_eq!(bench.kinds_of(pid), ["loaded", "runtime-exit"]);
        }

        #[test]
        fn without_its_file_the_library_is_inert_and_the_code_passes_through() {
            let bench = bench();
            let child = bench.child("k_exit", EXIT_THROUGH_THE_LINKER);
            let unset = Command::new(&child)
                .env("LD_PRELOAD", &bench.library)
                .env_remove(FILE_VAR)
                .status()
                .expect("run");
            assert_eq!(unset.code(), Some(7));
            let not_openable = Command::new(&child)
                .env("LD_PRELOAD", &bench.library)
                .env(FILE_VAR, bench.dir.path().join("absent").join(WITNESS_FILE))
                .status()
                .expect("run");
            assert_eq!(not_openable.code(), Some(7));
            assert!(!bench.witness.exists());
        }
    }
}
