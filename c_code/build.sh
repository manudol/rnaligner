#!/bin/sh

gcc -c viennarna.c -o viennarna.o -I/usr/include -fPIC
ar rcs libviennarna.a viennarna.o

