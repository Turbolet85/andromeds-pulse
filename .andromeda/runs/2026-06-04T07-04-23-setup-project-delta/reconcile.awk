BEGIN { skip=0; done_meta=0 }
(done_meta==0) && /^\*\*Last reconciled:\*\* / { sub(/^\*\*Last reconciled:\*\* /, "**Last reconciled:** " NOTE); done_meta=1; print; next }
/<!-- LIVING:api-surface:crate-triage start -->/ { print; while ((getline line < TFILE) > 0) print line; close(TFILE); skip=1; next }
/<!-- LIVING:api-surface:crate-triage end -->/ { skip=0; print; next }
skip==0 { print }
