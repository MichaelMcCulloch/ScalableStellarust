# Infrastructure to support microservices of stellarust app

## Why am I building a whole cloud for one little app?
Learning kubernetes + ansible the hard way


## What is required to deploy this infrastructure?
0. The default settings are 10 machines with 32 cores and 32GB RAM per machine. Adjust these numbers in the Vagrantfile to suit your machine

1. Arch or Manjaro:

    #### Packages:
    - vagrant
    - ovmf
    - libvirt

    #### Vagrant plugins:
    - vagrant-libvirt
    - vagrant-cachier

1. Network configuration (if you intend to navigate to the load balancer by name instead of IP address)
`echo server=/stellarust.com/10.11.0.1#53 > /etc/NetworkManager/dnsmasq.d/50-datacenter.conf` on your hypervisor host.

2. Create a separate storage pool named `stellarust` or delete the line `lv.storage_pool_name = "stellarust"` in the Vagrantfile.




## How to start the datacenter
A collection VSCode tasks exist to deploy the infrastructure. The `Infrastructure::E2E::Deploy` task will will deploy the infrastructure and the `Infrastructure::E2E::Redeploy` task will destroy the infrastructure and redeploy it.
