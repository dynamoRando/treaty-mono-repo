Use the script `make_cert.sh` (rename it from `make_cert.sh.bak` - it will need +x, i.e `chmod +x ./make_cert.sh`) to generate a local CA and certs for the test.

Inspect the script to ensure it has the proper values:

- DNS.1
- IP.1

for the cert you want to generate.

Then place it in this directory: `certs/` -- the `treaty/treaty-tests/src/harness` code will look for this `certs/` directory to pull the appropriate items for TLS testing. You'll need to copy the items from the `keys/` directory to here.


This script was taken from:

- https://stackoverflow.com/questions/76049656/unexpected-notvalidforname-with-rusts-tonic-with-tls


References:

- https://deliciousbrains.com/ssl-certificate-authority-for-local-https-development/
- https://github.com/FiloSottile/mkcert

