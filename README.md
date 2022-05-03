# Scaleable Stellarust

## Notes:
- All tasks are configured for VSCode.
- You can bring up the datacenter with `Infrastructure::Vagrant::Provision` task and deploy the cluster with `Infrastructure::Ansible::VirtualMachines::DeployDatacenter`.
- Your local dns server should search datacenter.local on the subnet XXX.XXX.XXX.XXX/YY. I achieved this by adding `server=/datacenter.local/XXX.XXX.XXX.XXX#53` to `/etc/NetworkManager/dnsmasq.d/50-datacenter.conf` on Manjaro