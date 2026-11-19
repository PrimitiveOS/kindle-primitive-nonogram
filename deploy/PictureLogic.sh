#!/bin/sh
# Name: Picture Logic
# Author: Primitive AI
# Icon: data:image/png;base64,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
# DontUseFBInk
# PrimitiveUSBLease

app=/mnt/us/picturelogic/bin/picturelogic-kindle
[ -x "$app" ] || exit 1
if [ -n "${KL_NATIVE_APP_RUN:-}" ]; then
    cp "$app" "$KL_NATIVE_APP_RUN/run" && chmod 755 "$KL_NATIVE_APP_RUN/run" || exit 1
    cd / || exit 1
    exec "$KL_NATIVE_APP_RUN/run"
fi

# Older launchers and the stock library start this entry without a shared RAM work directory.
case "$0" in
    /var/tmp/primitive-picturelogic.*/launch) work=${0%/launch} ;;
    *)
        work=$(mktemp -d /var/tmp/primitive-picturelogic.XXXXXX) || exit 1
        if ! cp "$0" "$work/launch"; then
            rmdir "$work"
            exit 1
        fi
        cd / || exit 1
        exec /bin/sh "$work/launch"
        ;;
esac
trap 'rm -rf "$work"' EXIT
cp "$app" "$work/run" && chmod 755 "$work/run" || exit 1
"$work/run"
