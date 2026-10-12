Question put at P4 (one fork, four options: App start (Recommended) · Workflow runner image · Harness preload · Stop at P4):

"Research names the cause in the app itself: it uses Xlib from two threads (GTK's main thread and a device-event thread the windowing library starts on X11) and never initialises Xlib's thread support. The runner's Xlib 1.7.5 does not do that at load; the dev host's 1.8.13 does. Measured on the dev host with the runner's Xlib package swapped in: 19 of 24 boots self-ended with the runner's call chain; with XInitThreads() called first in the app, 0 of 8. Where should the close land?"

The option chosen, as it was worded: "App start (Recommended) — Call XInitThreads() at the head of main() on Linux, before anything reaches Xlib. One small new module in pulse-app, through the libc crate it already depends on: no new dependency, no environment variable, no log record. It is the only option that removes the cause; the shipped app has the same defect on any host whose Xlib is older than 1.8 (Ubuntu 22.04 among them), and CI keeps measuring what ships. Cost: a product edit in main() for a window that leaves in Epoch 2."

The answer, verbatim:

App start (Recommended)

notes: Operator (pc overseer): it is the one option that removes the cause, and the defect is in what ships. Conditions for the plan: the call is the first thing main() does on Linux, before any thread exists and before any library that can reach Xlib; say what it does on a host where the library cannot be found (inert, never a failed start) and pin both arms; the module leaves with the window - name its leaving owner. The close is read on the runner: state how many boots over how many runs, and the rate an unclosed job would pass that count at, using the 7 of 8 run and the run on 95d17008 as the without arm. The swapped-library reading on this host goes into evidence with how the swap was made and undone.
