# The three readings on a spawned engine — 2026-10-10-engine-harness-proofs-completed

Printed by the three new arms of `pulse-app/tests/integration_console_engine.rs` on the final tree, 2026-10-10:

    cargo nextest run --workspace --profile ci --no-tests=fail --success-output immediate \
      -E 'binary(integration_console_engine) & (test(rejected) | test(listens) | test(lock_file))'

Summary line: `3 tests run: 3 passed, 4 skipped`. Each engine was the built `andromeda-pulse-engine run`, spawned
by its arm under a cleared environment on two claimed loopback ports, ended by SIGTERM by its pid. Each reading
is n = 1 per run; the same three arms also ran green in the three firings of the gate block and in the 14 hand runs
(`gate-readings.md`).

## Rejected port (`a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening`)

The gRPC variable held a claimed port's digits followed by `/tcp`.

    socket census, gRPC port variable rejected: Census { sockets: [Socket { table: "tcp", local: "127.0.0.1:30276", listening: true }, Socket { table: "unix", local: "(unnamed)", listening: false }, Socket { table: "unix", local: "(unnamed)", listening: false }], unlisted: 0 }
    gRPC bind record: level "ERROR", reason "invalid_port"

Asserted beside what is printed: exactly one `app.boot.otlp.grpc.bind` record, with no `bind_address`; exactly one
`app.boot.otlp.http.bind` record, INFO, its `bind_address` loopback at the HTTP port; a loopback connection to the
digits' port refused; the program still running, then ended by SIGTERM with its one `app.exit` record last.

## Socket census (`the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else`)

Both port variables valid.

    socket census, both ports valid: Census { sockets: [Socket { table: "tcp", local: "127.0.0.1:20825", listening: true }, Socket { table: "tcp", local: "127.0.0.1:27248", listening: true }, Socket { table: "unix", local: "(unnamed)", listening: false }, Socket { table: "unix", local: "(unnamed)", listening: false }], unlisted: 0 }

Two listening TCP sockets, both on `127.0.0.1`, at the two configured ports. No listening unix socket (the two
unix sockets are unnamed and not listening, a connected pair), no UDP socket, no IPv6 socket, and no socket inode
outside the five kernel tables. The census is read once no connection the readiness probe opened is still held.
In each arm it is first pointed at the test's own process holding a listener it opened and must find it; with the
fd-table read stubbed to empty, that control failed both arms (`red-readings.md`, reading 1).

## Lock file's directory (`the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives`)

    lock file, temp dir given, no runtime dir: andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock, 0 bytes, mode 600
    lock file, runtime dir given: andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock, 0 bytes, mode 600

Asserted beside what is printed: in each environment the file is the only entry of the directory the rule gives
(the supplied temp dir; with a runtime dir supplied, that directory, the supplied temp dir then empty); the data
dir's top entries are exactly `corpus`, `logs`, `run`; no file under the data dir carries the lock file's name
prefix; the lock file's name, the lock directory's path as given and as resolved each appear 0 times in the run's
log family.

## A reading for the operator, not repaired here

Under the environment the four existing arms and the engine cycle hand the engine (no temp-dir variable, no
runtime dir), the rule yields the shared temp dir. Read on this host after the gate block: the file
`andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock` stands in the system temp dir. It is the case research's hand
reading recorded and the directive names as `One place on a node`'s; the environment of those arms and of the
cycle's children is unchanged by this chunk.
