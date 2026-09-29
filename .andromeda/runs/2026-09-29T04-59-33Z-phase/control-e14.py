import os
import socket
import subprocess

# Known-positive control for gate e14 (the leg's non-priming precondition probe):
# with :4317 accepting, the probe must exit non-zero.
srv = socket.socket()
srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
srv.bind(("127.0.0.1", 4317))
srv.listen(1)
probe = (
    "test -f target/release/pulse-app.exe && python -X utf8 -c \"import socket,sys; "
    "sys.exit(sum(socket.socket().connect_ex(('127.0.0.1',p))==0 for p in (4317,4318)))\""
)
BASH = os.environ["SHELL"]  # the shell gate.py resolves ($SHELL); a bare "bash" reaches WSL here
rc = subprocess.run([BASH, "-o", "pipefail", "-c", probe]).returncode
srv.close()
print(f"control-e14: probe exit with :4317 accepting = {rc} (must be non-zero)")
