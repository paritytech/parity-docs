---
title: Monitor and Harden a Node
description: Monitor a Polkadot SDK node with Prometheus, Grafana, and Alertmanager, tune the Linux kernel for latency, and use the node's sandboxing features.
categories: Nodes
---

# Monitor and Harden a Node

## Introduction

Every Polkadot SDK node binary, including `polkadot`, `polkadot-parachain`, and `polkadot-omni-node`, exposes Prometheus metrics and runs on a standard Linux host. This page covers the operational tasks that apply to any node, whichever mode it runs in:

- Collect metrics with Prometheus, visualize them in Grafana, and send alerts with Alertmanager.
- Tune the Linux kernel to reduce latency on dedicated hosts.
- Harden the host and use the node's built-in sandboxing (secure-validator mode).

Validator-specific practices, such as session key custody and staking operations, are network topics. See [General Management](https://developers.polkadot.com/node-infrastructure/run-a-validator/operational-tasks/general-management/) on the Polkadot Developer Docs.

## Monitor Your Node

Every node binary serves Prometheus metrics on port `9615` of `localhost` by default. Add `--prometheus-external` to serve them on all interfaces, or `--prometheus-port` to change the port. The following tools collect, display, and alert on these metrics:

- [Prometheus](https://prometheus.io/): An open-source monitoring toolkit for collecting and querying time-series data.
- [Grafana](https://grafana.com/): A visualization tool for real-time metrics, providing interactive dashboards.
- [Alertmanager](https://prometheus.io/docs/alerting/latest/alertmanager/): A tool for managing and routing alerts based on Prometheus data.

This section covers setting up these tools and configuring alerts to notify you of potential issues.

### Environment Setup

Before installing Prometheus, ensure the environment is set up securely by running Prometheus with restricted user privileges.

Follow these steps:

1. Create a Prometheus user to ensure Prometheus runs with minimal permissions:

    ```bash
    sudo useradd --no-create-home --shell /usr/sbin/nologin prometheus
    ```

2. Create directories for configuration and data storage:

    ```bash
    sudo mkdir /etc/prometheus
    sudo mkdir /var/lib/prometheus
    ```
  
3. Change directory ownership to ensure Prometheus has access:

    ```bash
    sudo chown -R prometheus:prometheus /etc/prometheus
    sudo chown -R prometheus:prometheus /var/lib/prometheus
    ```

### Install and Configure Prometheus

After setting up the environment, install and configure the latest version of Prometheus as follows:

1. Download Prometheus for your system architecture from the [releases page](https://github.com/prometheus/prometheus/releases/). Replace `INSERT_RELEASE_DOWNLOAD_LINK` with the release binary URL (for example, `https://github.com/prometheus/prometheus/releases/download/v3.0.0/prometheus-3.0.0.linux-amd64.tar.gz`):

    ```bash
    sudo apt-get update && sudo apt-get upgrade
    wget INSERT_RELEASE_DOWNLOAD_LINK
    tar xfz prometheus-*.tar.gz
    cd prometheus-3.0.0.linux-amd64
    ```

2. Set up Prometheus:

    1. Copy binaries:

        ```bash
        sudo cp ./prometheus /usr/local/bin/
        sudo cp ./promtool /usr/local/bin/
        ```

    2. Copy directories and assign ownership of these files to the `prometheus` user:

        ```bash
        sudo cp -r ./consoles /etc/prometheus
        sudo cp -r ./console_libraries /etc/prometheus
        sudo chown -R prometheus:prometheus /etc/prometheus/consoles
        sudo chown -R prometheus:prometheus /etc/prometheus/console_libraries
        ```

    3. Clean up the download directory:

        ```bash
        cd .. && rm -r prometheus*
        ```

3. Create `prometheus.yml` to define global settings, rule files, and scrape targets:

    ```bash
    sudo nano /etc/prometheus/prometheus.yml
    ```

    ```yaml title="prometheus-config.yml"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/prometheus-config.yml'
    ```

    This example configuration scrapes Prometheus itself and the node's metrics endpoint on port `9615` every five seconds.

4. Verify the configuration with `promtool`, an open source monitoring tool:

    ```bash
    promtool check config /etc/prometheus/prometheus.yml
    ```

5. Save the configuration and change the ownership of the file to `prometheus` user:

    ```bash
    sudo chown prometheus:prometheus /etc/prometheus/prometheus.yml
    ```

### Start Prometheus

1. Launch Prometheus with the appropriate configuration file, storage location, and necessary web resources, running it with restricted privileges for security:

    ```bash
    sudo -u prometheus /usr/local/bin/prometheus --config.file /etc/prometheus/prometheus.yml \
    --storage.tsdb.path /var/lib/prometheus/ \
    --web.console.templates=/etc/prometheus/consoles \
    --web.console.libraries=/etc/prometheus/console_libraries
    ```

    If you set the server up properly, you should see terminal output similar to the following:

    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/terminal-output-01.html'

2. Verify you can access the Prometheus interface by navigating to:

    ```text
    http://INSERT_SERVER_IP:9090/graph
    ```

    If the interface appears to work as expected, exit the process using `Control + C`.

3. Create a systemd service file to ensure Prometheus starts on boot:

    ```bash
    sudo nano /etc/systemd/system/prometheus.service
    ```

    ```bash title="prometheus.service"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/systemd-config.md'
    ```

4. Reload systemd and enable the service to start on boot:

    ```bash
    sudo systemctl daemon-reload && sudo systemctl enable prometheus && sudo systemctl start prometheus
    ```

5. Verify the service is running by visiting the Prometheus interface again at:

    ```text
    http://INSERT_SERVER_IP:9090/
    ```

### Install and Configure Grafana

This guide follows [Grafana's canonical installation instructions](https://grafana.com/docs/grafana/latest/setup-grafana/installation/debian/#install-from-apt-repository).

To install and configure Grafana, follow these steps:

1. Install Grafana prerequisites:

    ```bash
    sudo apt-get install -y apt-transport-https software-properties-common wget    
    ```

2. Import the [GPG key](https://gnupg.org/):

    ```bash
    sudo mkdir -p /etc/apt/keyrings/
    wget -q -O - https://apt.grafana.com/gpg.key | gpg --dearmor | sudo tee /etc/apt/keyrings/grafana.gpg > /dev/null
    ```

3. Configure the stable release repo and update packages:

    ```bash
    echo "deb [signed-by=/etc/apt/keyrings/grafana.gpg] https://apt.grafana.com stable main" | sudo tee -a /etc/apt/sources.list.d/grafana.list
    sudo apt-get update
    ```

4. Install the latest stable version of Grafana:

    ```bash
    sudo apt-get install grafana
    ```

To configure Grafana, take these steps:

1. Configure Grafana to start automatically on boot and start the service:

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl enable grafana-server.service
    sudo systemctl start grafana-server
    ```

2. Check if Grafana is running:

    ```bash
    sudo systemctl status grafana-server
    ```

    If necessary, you can stop or restart the service with the following commands:

    ```bash
    sudo systemctl stop grafana-server
    sudo systemctl restart grafana-server
    ```

3. Access Grafana by navigating to the following URL and logging in with the default username and password (`admin`):

    ```text
    http://INSERT_SERVER_IP:3000/login
    ```

    !!! tip "Change default port"
        To change Grafana's port, edit `/usr/share/grafana/conf/defaults.ini`:

        ```bash
        sudo vim /usr/share/grafana/conf/defaults.ini
        ```

        Modify the `http_port` value, then restart Grafana:

        ```bash
        sudo systemctl restart grafana-server
        ```

![Grafana login screen](/images/node-infrastructure/run-a-validator/operational-tasks/general-management/general-management-01.webp)

To visualize node metrics, follow these steps:

1. Select the gear icon to access **Data Sources** settings.
2. Select **Add data source** to define the data source.

    ![Select Prometheus](/images/node-infrastructure/run-a-validator/operational-tasks/general-management/general-management-02.webp)

3. Select **Prometheus**.

    ![Save and test](/images/node-infrastructure/run-a-validator/operational-tasks/general-management/general-management-03.webp)

4. Enter `http://localhost:9090` in the **URL** field and click **Save & Test**. If **"Data source is working"** appears, your connection is configured correctly.

    ![Import dashboard](/images/node-infrastructure/run-a-validator/operational-tasks/general-management/general-management-04.webp)

5. Select **Import** from the left menu, choose **Prometheus** from the dropdown, and click **Import**.

6. Start your node. You can now monitor node performance, block height, network traffic, and tasks on the Grafana dashboard.

    ![Live dashboard](/images/node-infrastructure/run-a-validator/operational-tasks/general-management/general-management-05.webp)

The [Grafana dashboards](https://grafana.com/grafana/dashboards) page features user created dashboards made available for public use. For an example, see the [Substrate Node Metrics](https://grafana.com/grafana/dashboards/21715-substrate-node-metrics/) dashboard.

### Install and Configure Alertmanager

[Alertmanager](https://prometheus.io/docs/alerting/latest/alertmanager/) is an optional component that complements Prometheus by managing alerts and notifying users about potential issues.

Follow these steps to install and configure Alertmanager:

1. Download Alertmanager for your system architecture from the [releases page](https://github.com/prometheus/alertmanager/releases). Replace `INSERT_RELEASE_DOWNLOAD_LINK` with the release binary URL (for example, `https://github.com/prometheus/alertmanager/releases/download/v0.28.0-rc.0/alertmanager-0.28.0-rc.0.linux-amd64.tar.gz`):

    ```bash
    wget INSERT_RELEASE_DOWNLOAD_LINK
    tar -xvzf alertmanager*
    ```

2. Copy the binaries to the system directory and set permissions:

    ```bash
    cd alertmanager-0.28.0-rc.0.linux-amd64
    sudo cp ./alertmanager /usr/local/bin/
    sudo cp ./amtool /usr/local/bin/
    sudo chown prometheus:prometheus /usr/local/bin/alertmanager
    sudo chown prometheus:prometheus /usr/local/bin/amtool
    ```

3. Create the `alertmanager.yml` configuration file under `/etc/alertmanager`:

    ```bash
    sudo mkdir /etc/alertmanager
    sudo nano /etc/alertmanager/alertmanager.yml
    ```

    Generate an [app password in your Google account](https://support.google.com/accounts/answer/185833?hl=en) to enable email notifications from Alertmanager. Then, add the following code to the configuration file to define email notifications using your  email and app password: 

    ```yml title="alertmanager.yml"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/alertmanager.yml'
    ```


    ```bash
    sudo chown -R prometheus:prometheus /etc/alertmanager
    ```

4. Configure Alertmanager as a service by creating a systemd service file:

    ```bash
    sudo nano /etc/systemd/system/alertmanager.service
    ```

    ```yml title="alertmanager.service"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/systemd-alert-config.md'
    ```

5. Reload and enable the service:

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl enable alertmanager
    sudo systemctl start alertmanager
    ```

6. Verify the service status:

    ```bash
    sudo systemctl status alertmanager
    ```

    If you have configured Alertmanager properly, the **Active** field should display **active (running)** similar to below:

    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/alertmanager-status.html'

#### Grafana Plugin

There is an [Alertmanager plugin in Grafana](https://grafana.com/grafana/plugins/alertmanager/) that can help you monitor alert information.

Follow these steps to use the plugin:

1. Install the plugin:

    ```bash
    sudo grafana-cli plugins install camptocamp-prometheus-alertmanager-datasource
    ```

2. Restart Grafana:

    ```bash
    sudo systemctl restart grafana-server
    ```

3. Configure Alertmanager as a data source in your Grafana dashboard (`INSERT_SERVER_IP:3000`):

    1. Go to **Configuration** > **Data Sources** and search for **Prometheus Alertmanager**.
    2. Enter the server URL and port for the Alertmanager service, and select **Save & Test** to verify the connection.

4. Import the [8010](https://grafana.com/grafana/dashboards/8010-prometheus-alertmanager/) dashboard for Alertmanager, selecting **Prometheus Alertmanager** in the last column, then select **Import**.

#### Integrate Alertmanager

Complete the integration by following these steps to enable communication between Prometheus and Alertmanager and configure detection and alert rules:

1. Update the `etc/prometheus/prometheus.yml` configuration file to include the following code:

    ```yml title="prometheus.yml"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/update-prometheus.yml:5:12'
    ```

    Expand the following item to view the complete `prometheus.yml` file.

    ??? code "prometheus.yml"

        ```yml title="prometheus.yml"
        --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/update-prometheus.yml'
        ```

2. Create the rules file for detection and alerts:

    ```bash
    sudo nano /etc/prometheus/rules.yml
    ```

    Add a sample rule to trigger email notifications for node downtime over five minutes:

    ```yml title="rules.yml"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/instance-down.yml'
    ```

    If any of the conditions defined in the rules file are met, an alert will be triggered. For more on alert rules, refer to [Alerting Rules](https://prometheus.io/docs/prometheus/latest/configuration/alerting_rules/) and [additional alerts](https://samber.github.io/awesome-prometheus-alerts/rules/).

3. Update the file ownership to `prometheus`:

    ```bash
    sudo chown prometheus:prometheus rules.yml
    ```

4. Validate the rules syntax:

    ```bash
    sudo -u prometheus promtool check rules rules.yml
    ```

5. Restart Prometheus and Alertmanager:

    ```bash
    sudo systemctl restart prometheus && sudo systemctl restart alertmanager
    ```

You now receive an email alert whenever a rule's conditions are met.

## Tune the Kernel

On a dedicated host, the following kernel settings can improve responsiveness, reduce latency, and keep performance consistent under load. They matter most for latency-sensitive nodes such as validators and collators.

### Deactivate Simultaneous Multithreading

Validators run critical tasks primarily in single-threaded mode, so optimizing single-core CPU performance can reduce latency and improve stability. Deactivating simultaneous multithreading (SMT) can prevent virtual cores from affecting performance. SMT is called Hyper-Threading on Intel and 2-way SMT on AMD Zen.

Take the following steps to deactivate every other (vCPU) core:

1. Loop through all the CPU cores and deactivate the virtual cores associated with them:

    ```bash
    for cpunum in $(cat /sys/devices/system/cpu/cpu*/topology/thread_siblings_list | \
    cut -s -d, -f2- | tr ',' '\n' | sort -un)
    do
    echo 0 > /sys/devices/system/cpu/cpu$cpunum/online
    done
    ```

2. To permanently save the changes, add `nosmt=force` to the `GRUB_CMDLINE_LINUX_DEFAULT` variable in `/etc/default/grub`:

    ```bash
    sudo nano /etc/default/grub
    # Add to GRUB_CMDLINE_LINUX_DEFAULT
    ```

    ```config title="/etc/default/grub"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/grub-config-01.js:1:7'
    ```

3. Update GRUB to apply changes:

    ```bash
    sudo update-grub
    ```

4. After the reboot, you should see that half of the cores are offline. To confirm, run:

    ```bash
    lscpu --extended
    ```

### Deactivate Automatic NUMA Balancing

Deactivating Non-Uniform Memory Access (NUMA) balancing for multi-CPU setups helps keep processes on the same CPU node, minimizing latency.

Follow these steps:

1. Deactivate NUMA balancing in runtime:

    ```bash
    sysctl kernel.numa_balancing=0
    ```

2. Deactivate NUMA balancing permanently by adding `numa_balancing=disable` to the GRUB settings:

    ```bash
    sudo nano /etc/default/grub
    # Add to GRUB_CMDLINE_LINUX_DEFAULT
    ```

    ```config title="/etc/default/grub"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/grub-config-01.js:9:15'
    ```

3. Update GRUB to apply changes:

    ```bash
    sudo update-grub
    ```

4. Confirm the deactivation:

    ```bash
    sysctl -a | grep 'kernel.numa_balancing'
    ```

If you successfully deactivated NUMA balancing, the preceding command should return `0`.

### Spectre and Meltdown Mitigations

[Spectre](https://en.wikipedia.org/wiki/Spectre_(security_vulnerability)) and [Meltdown](https://en.wikipedia.org/wiki/Meltdown_(security_vulnerability)) are well-known CPU vulnerabilities that exploit speculative execution to access sensitive data. These vulnerabilities have been patched in recent Linux kernels, but the mitigations can slightly impact performance, especially in high-throughput or containerized environments.

If your security requirements allow it, you can deactivate specific mitigations, such as Spectre V2 and Speculative Store Bypass Disable (SSBD), to improve performance.

To selectively deactivate the Spectre mitigations, take these steps:

1. Update the `GRUB_CMDLINE_LINUX_DEFAULT` variable in your `/etc/default/grub` configuration:

    ```bash
    sudo nano /etc/default/grub
    # Add to GRUB_CMDLINE_LINUX_DEFAULT
    ```

    ```config title="/etc/default/grub"
    --8<-- 'code/node-infrastructure/run-a-validator/operational-tasks/general-management/grub-config-01.js:17:23'
    ```

2. Update GRUB to apply changes and then reboot:

    ```bash
    sudo update-grub
    sudo reboot
    ```

This approach selectively deactivates the Spectre V2 and Spectre V4 mitigations, leaving other protections intact. For full security, keep mitigations activated unless there's a significant performance need, as disabling them could expose the system to potential attacks on affected CPUs.

## Harden the Host

A node's host should expose as little as possible. Follow these Linux practices:

- Run the node as a non-root user.
- Apply OS security patches regularly.
- Enable and configure a firewall, and open only the node's P2P port and any RPC or metrics ports you intend to expose.
- Use key-based SSH authentication and deactivate password-based login.
- Back up data regularly and harden your SSH configuration. See this [SSH hardening guide](https://blog.stribik.technology/2015/01/04/secure-secure-shell.html) for details.
- Start the node at boot and restart it if it stops, for example with a systemd service.

### Secure-Validator Mode

The `polkadot` binary includes secure-validator mode, which sandboxes the worker processes that prepare and execute parachain validation code. The sandbox applies filesystem, networking, and process restrictions. The node turns it on automatically when it runs with `--validator`.

At startup, the node checks that the host supports the required kernel features:

- **Linux on x86-64**: The sandbox is available only on Linux, and `seccomp` filtering only on CPUs from the x86-64 family (usually Intel or AMD).
- **`seccomp`**: Restricts the system calls the worker processes can make. Check that it is enabled in your kernel:

    ```bash
    cat /boot/config-`uname -r` | grep CONFIG_SECCOMP=
    ```

    If `seccomp` is enabled, the output is similar to the following:

    ```text
    CONFIG_SECCOMP=y
    ```

- **Filesystem sandboxing**: Either [Landlock](https://docs.kernel.org/userspace-api/landlock.html) (Linux 5.13 or later) or the ability to unshare the user namespace and change root. See [Validator Mode](/nodes/run-a-node/relay-chain/full-node/#validator-mode) for the Landlock check.

If a required feature is missing, the node logs an error and refuses to start in validator mode. The `--insecure-validator-i-know-what-i-do` flag lets it run anyway, with the available features applied on a best-effort basis. Use it only when you understand the risk.

For key management, signing, and other validator practices, see [General Management](https://developers.polkadot.com/node-infrastructure/run-a-validator/operational-tasks/general-management/) on the Polkadot Developer Docs.
