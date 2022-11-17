## PHP у Rust

async rust
php + cgi
app server

other features:
* static file serving
* thread/forking?
* routing/request rewriting
* logging (optimize for journald)

maybe:
* metrics
* log routing/filtering
* uwsgi/fastcgi wire protocol

focus:
* linux, systemd - I don't care for the rest
* should be easily chroot-able, running under DynamicUser=true
* one process - one application


Inspired by uwsgi, but simpler.
