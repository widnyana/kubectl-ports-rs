<h1 align="center">kubectl-ports</h1>
<p align="center">
  <em>🕸 Show exposed container ports in the cluster.</em>
</p>

`kubectl-ports` makes use of the `kube-rs` SDK. It will retrieve running pods and filters out the container ports information of each pod, then prints out the final result in a table view.

## 🔰 Installation

A several mechanism to install the plugin is available, pick one that easiest for you.

### Using released binary

Download your binary here: https://github.com/widnyana/kubectl-ports-rs/releases/latest, extract, and copy the file(s) into `$HOME/.local/bin`s

I use github action to build the binary for following platform:
- `aarch64-apple-darwin`
- `aarch64-unknown-linux-musl`
- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl`


### Compiling on your own

```shell
$ git clone https://github.com/widnyana/kubectl-ports-rs.git kubectl-ports-rs
$ cd kubectl-ports-rs && make build install clean
$ kubectl ports --help
```


## 📝 Usage

```shell
~ 🐶 kubectl-ports --help
A kubectl plugin to provide a list of exposed ports on kubernetes Pod resources

Usage: kubectl-ports [OPTIONS] [RESOURCE]

Arguments:
  [RESOURCE]
          select Kubernetes resource you want to list the port(s)

          [default: pod]

          Possible values:
          - pod:     (default) Enable both coloring and formatting
          - service: Apply syntax highlighting to output
          - svc

Options:
      --context <CONTEXT>
          The name of the kubeconfig context to use

  -n, --namespace <NAMESPACE>
          Show only pods from this namespace

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## 🔖 Glances

### Listing ports exposed by Pod

```shell
~$ kubectl ports -n kube-system
  +===============+==========================================+========================+==================+===================+
  |   Namespace   |                 Pod Name                 |     Container name     |  Container Port  |     Port Name     |
  +===============+==========================================+========================+==================+===================+
  |  kube-system  |  fluentbit-gke-46dgf                     |  fluentbit             |  2020/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-46dgf                     |  fluentbit-gke         |  2021/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-4f4tv                     |  fluentbit             |  2020/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-4f4tv                     |  fluentbit-gke         |  2021/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-gbklt                     |  fluentbit             |  2020/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-gbklt                     |  fluentbit-gke         |  2021/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-pv6jv                     |  fluentbit             |  2020/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-pv6jv                     |  fluentbit-gke         |  2021/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-shstp                     |  fluentbit             |  2020/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  fluentbit-gke-shstp                     |  fluentbit-gke         |  2021/TCP        |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-99wvm               |  gke-metadata-server   |  987/TCP         |  alts             |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-99wvm               |  gke-metadata-server   |  988/TCP         |  metadata-server  |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-99wvm               |  gke-metadata-server   |  989/TCP         |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-dqvnm               |  gke-metadata-server   |  987/TCP         |  alts             |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-dqvnm               |  gke-metadata-server   |  988/TCP         |  metadata-server  |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-dqvnm               |  gke-metadata-server   |  989/TCP         |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-qwlsc               |  gke-metadata-server   |  987/TCP         |  alts             |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-qwlsc               |  gke-metadata-server   |  988/TCP         |  metadata-server  |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-qwlsc               |  gke-metadata-server   |  989/TCP         |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-vqwmc               |  gke-metadata-server   |  987/TCP         |  alts             |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-vqwmc               |  gke-metadata-server   |  988/TCP         |  metadata-server  |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-vqwmc               |  gke-metadata-server   |  989/TCP         |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-wlc94               |  gke-metadata-server   |  987/TCP         |  alts             |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-wlc94               |  gke-metadata-server   |  988/TCP         |  metadata-server  |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  gke-metadata-server-wlc94               |  gke-metadata-server   |  989/TCP         |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  kubedns               |  10053/UDP       |  dns-local        |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  kubedns               |  10053/TCP       |  dns-tcp-local    |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  kubedns               |  10055/TCP       |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  dnsmasq               |  53/UDP          |  dns              |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  dnsmasq               |  53/TCP          |  dns-tcp          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-4zpkq               |  sidecar               |  10054/TCP       |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  kubedns               |  10053/UDP       |  dns-local        |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  kubedns               |  10053/TCP       |  dns-tcp-local    |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  kubedns               |  10055/TCP       |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  dnsmasq               |  53/UDP          |  dns              |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  dnsmasq               |  53/TCP          |  dns-tcp          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  kube-dns-859b68dc67-lbqvs               |  sidecar               |  10054/TCP       |  metrics          |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  l7-default-backend-6b99559c7d-bhgw5     |  default-http-backend  |  8080/TCP        |                   |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  metrics-server-64cf6869bd-7nrdg         |  metrics-server        |  4443/TCP        |  https            |
  +---------------+------------------------------------------+------------------------+------------------+-------------------+
  |  kube-system  |  metrics-server-v0.5.2-866bc7fbf8-bzrvc  |  metrics-server        |  10250/TCP       |  https            |
  +===============+==========================================+========================+==================+===================+

```

### Listing ports exposed by Service

```shell
~$ kubectl ports -n kube-system svc
  +===============+========================+=============+=================+=============+===============+================+=============+
  |   Namespace   |      Service Name      |    Type     |   Cluster IP    |  Port Name  |  Target Port  |  Exposed Port  |  Node Port  |
  +===============+========================+=============+=================+=============+===============+================+=============+
  |  kube-system  |  default-http-backend  |  NodePort   |  172.23.5.200   |    http     |  8080/TCP     |  80/TCP        |  32725/TCP  |
  +---------------+------------------------+-------------+-----------------+-------------+---------------+----------------+-------------+
  |  kube-system  |  kube-dns              |  ClusterIP  |  172.23.0.10    |     dns     |  53/UDP       |  53/UDP        |  0/UDP      |
  +---------------+------------------------+-------------+-----------------+-------------+---------------+----------------+-------------+
  |  kube-system  |  kube-dns              |  ClusterIP  |  172.23.0.10    |   dns-tcp   |  53/TCP       |  53/TCP        |  0/TCP      |
  +---------------+------------------------+-------------+-----------------+-------------+---------------+----------------+-------------+
  |  kube-system  |  metrics-server        |  ClusterIP  |  172.23.11.137  |      -      |  https/TCP    |  443/TCP       |  0/TCP      |
  +===============+========================+=============+=================+=============+===============+================+=============+
```


## 📃 License

MIT OR Apache-2.0 [©widnyana](https://github.com/widnyana)
