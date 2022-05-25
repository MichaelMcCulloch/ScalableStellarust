#!/bin/sh
# generate CA private and public keys
openssl genrsa -aes256 -out ca-key.pem 4096
openssl req -new -x509 -days 365 -key ca-key.pem -sha256 -out ca.pem
# Now that you have a CA, you can create a server key and certificate signing request (CSR). Make sure that “Common Name” matches the hostname you use to connect to Docker:
openssl genrsa -out server-key.pem 4096
openssl req -subj "/CN=workstation1" -sha256 -new -key server-key.pem -out server.csr
#Next, we’re going to sign the public key with our CA:
echo subjectAltName = DNS:workstation1,IP:10.10.10.20,IP:127.0.0.1 >> extfile.cnf
#Set the Docker daemon key’s extended usage attributes to be used only for server authentication:
echo extendedKeyUsage = serverAuth >> extfile.cnf
#Now, generate the signed certificate
openssl x509 -req -days 365 -sha256 -in server.csr -CA ca.pem -CAkey ca-key.pem -CAcreateserial -out server-cert.pem -extfile extfile.cnf
#For client authentication, create a client key and certificate signing request:
openssl genrsa -out key.pem 4096
openssl req -subj '/CN=client' -new -key key.pem -out client.csr

#To make the key suitable for client authentication, create a new extensions config file:
echo extendedKeyUsage = clientAuth > extfile-client.cnf

#Now, generate the signed certificate
openssl x509 -req -days 365 -sha256 -in client.csr -CA ca.pem -CAkey ca-key.pem -CAcreateserial -out cert.pem -extfile extfile-client.cnf

#After generating cert.pem and server-cert.pem you can safely remove the two certificate signing requests and extensions config files
rm -v client.csr server.csr extfile.cnf extfile-client.cnf

#To protect your keys from accidental damage, remove their write permissions. To make them only readable by you, change file modes as follows:
chmod -v 0400 ca-key.pem key.pem server-key.pem