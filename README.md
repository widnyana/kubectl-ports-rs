<h1 align="center">kubectl-ports</h1>
<p align="center">
  <em>🕸 Show exposed container ports in the cluster.</em>
</p>

kubectl-ports makes use of the `kube-rs` SDK. It will retrieve running pods and filters out the
container ports information of each pod, then prints out the final result in a table view.

### 🔰 Installation


```shell
$ git clone https://github.com/widnyana/kubectl-ports-rs.git
$ cd kubectl-ports-rs && make build install
$ kubectl ports --help
```


### 📝 Usage

```shell
~ 🐶 kubectl-ports --help
A kubectl krew plugin to provide a list of exposed ports on kubernetes Pod resources

Usage: kubectl-ports [OPTIONS]

Options:
      --context <CONTEXT>      The name of the kubeconfig context to use
  -n, --namespace <NAMESPACE>  Show only pods from this namespace
  -h, --help                   Print help
  -V, --version                Print version

```

### 🔖 Glances

```shell
kubectl ports -n kube-system
+-------------+----------------------------------------+----------------------+----------------+-----------------+
|  Namespace  |                Pod Name                |    Container name    | Container Port |    Port Name    |
+-------------+----------------------------------------+----------------------+----------------+-----------------+
| kube-system | fluentbit-gke-wgmsk                    | fluentbit            | 2020/TCP       | metrics         |
| kube-system | fluentbit-gke-wgmsk                    | fluentbit-gke        | 2021/TCP       | metrics         |
| kube-system | fluentbit-gke-zngzv                    | fluentbit            | 2020/TCP       | metrics         |
| kube-system | fluentbit-gke-zngzv                    | fluentbit-gke        | 2021/TCP       | metrics         |
| kube-system | gke-metadata-server-g9zk8              | gke-metadata-server  | 987/TCP        | alts            |
| kube-system | gke-metadata-server-g9zk8              | gke-metadata-server  | 988/TCP        | metadata-server |
| kube-system | gke-metadata-server-g9zk8              | gke-metadata-server  | 989/TCP        | metrics         |
| kube-system | gke-metadata-server-v45zx              | gke-metadata-server  | 987/TCP        | alts            |
| kube-system | gke-metadata-server-v45zx              | gke-metadata-server  | 988/TCP        | metadata-server |
| kube-system | gke-metadata-server-v45zx              | gke-metadata-server  | 989/TCP        | metrics         |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | kubedns              | 10053/UDP      | dns-local       |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | kubedns              | 10053/TCP      | dns-tcp-local   |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | kubedns              | 10055/TCP      | metrics         |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | dnsmasq              | 53/UDP         | dns             |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | dnsmasq              | 53/TCP         | dns-tcp         |
| kube-system | kube-dns-7d44cdb5d5-7trqx              | sidecar              | 10054/TCP      | metrics         |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | kubedns              | 10053/UDP      | dns-local       |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | kubedns              | 10053/TCP      | dns-tcp-local   |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | kubedns              | 10055/TCP      | metrics         |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | dnsmasq              | 53/UDP         | dns             |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | dnsmasq              | 53/TCP         | dns-tcp         |
| kube-system | kube-dns-7d44cdb5d5-tfcgc              | sidecar              | 10054/TCP      | metrics         |
| kube-system | l7-default-backend-6dc845c45d-hn9hq    | default-http-backend | 8080/TCP       |                 |
| kube-system | metrics-server-v0.5.2-6bf845b67f-7mnbv | metrics-server       | 10250/TCP      | https           |
| kube-system | node-local-dns-4qsdp                   | node-cache           | 53/UDP         | dns             |
| kube-system | node-local-dns-4qsdp                   | node-cache           | 53/TCP         | dns-tcp         |
| kube-system | node-local-dns-4qsdp                   | node-cache           | 9253/TCP       | metrics         |
| kube-system | node-local-dns-lrrhm                   | node-cache           | 53/UDP         | dns             |
| kube-system | node-local-dns-lrrhm                   | node-cache           | 53/TCP         | dns-tcp         |
| kube-system | node-local-dns-lrrhm                   | node-cache           | 9253/TCP       | metrics         |
+-------------+----------------------------------------+----------------------+----------------+-----------------+

```

### 📃 License

MIT [©widnyana](https://github.com/widnyana)
