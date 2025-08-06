#!/bin/sh
for ID in $(xsetwacom list | cut -f2 | cut -d' ' -f2); do xsetwacom set "$ID" maptooutput next; done
