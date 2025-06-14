# NetPulse

NetPulse is a bandwidth monitoring server written in Rust.

---

## Features

- Interactive dashboard with day/week/month/year charts
- Export charts as image or PDF
- Data stored in SQLite
- Easy packaging: DEB and RPM available from GitHub Release page

---

## Quick Start

### 1. **Build and Run**

```sh
cargo build --release
./target/release/netpulse
```

### 2. **Test Data Generation**

To generate test data (12 measurements per day for the last year):

```sh
cargo run --bin generate_test_data
```

This will populate `netpulse.db` with realistic data for all chart ranges.

---

## Packaging

### **DEB and RPM Packages**

- Pre-built `.deb` and `.rpm` packages are available on the [GitHub Releases page](https://github.com/afinck/netpulse/releases).
- These are built automatically by GitHub Actions on every tagged release.

---

## Database Optimization

On startup, NetPulse will automatically create an index on the `timestamp` column for fast queries:

```sql
CREATE INDEX IF NOT EXISTS idx_measurements_timestamp ON measurements(timestamp);
```

You can also run this manually:

```sh
sqlite3 netpulse.db "CREATE INDEX IF NOT EXISTS idx_measurements_timestamp ON measurements(timestamp);"
```

---

## Development

- The workspace is set up for use in a dev container on Debian Bullseye.
- To open documentation in your host browser, use:
  ```sh
  $BROWSER <url>
  ```

This command will open the specified webpage in your host machine’s default browser from inside your dev container.

---

## Dashboard

- The dashboard is available at `http://localhost:3000/` (or your configured port).
- Use the range buttons to view day, week, month, or year charts.
- Export the current chart as an image or PDF using the buttons below the chart.

---

## License

MIT

---

## Links

- [Chart.js Documentation](https://www.chartjs.org/docs/latest/)
- [jsPDF Documentation](https://github.com/parallax/jsPDF)

---

## System Dependencies

Before running or packaging NetPulse, ensure these system packages are installed:

- `sqlite3` and `libsqlite3-dev` (for SQLite database support)
- `speedtest` (Ookla Speedtest CLI, for bandwidth measurement)

On Debian/Ubuntu, install with:

```sh
sudo apt update
sudo apt install sqlite3 libsqlite3-dev
curl -s https://install.speedtest.net/app/cli/install.deb.sh | sudo bash
sudo apt install speedtest
```



## 🚀 Raspberry Pi 4 (aarch64) Build & Install

### 1. Build for Raspberry Pi 4 (aarch64/64-bit)

If you want to build Netpulse for a 64-bit Raspberry Pi OS:

```sh
# On your PC or in CI:
rustup target add aarch64-unknown-linux-gnu
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target=aarch64-unknown-linux-gnu
# Or to build a .deb package:
cargo install cargo-deb
cargo deb --target=aarch64-unknown-linux-gnu
```

You can also use our [GitHub Actions workflow](.github/workflows/...) to automatically build `.deb` packages for aarch64.

---

### 2. Install Ookla Speedtest CLI

Netpulse requires the Ookla Speedtest CLI.  
**It is not available in the default Raspberry Pi OS repositories.**  
Install it manually:

```sh
curl -Lo speedtest.tgz https://install.speedtest.net/app/cli/ookla-speedtest-1.2.0-linux-aarch64.tgz
tar -xzf speedtest.tgz
sudo mv speedtest /usr/local/bin/
sudo chmod +x /usr/local/bin/speedtest
rm speedtest.tgz
```

---

## Other Installation Steps

- Install `sqlite3`:
  ```sh
  sudo apt-get install sqlite3
  ```

- Install Netpulse `.deb` (replace with your actual file name):
  ```sh
  sudo apt install ./netpulse_0.1.0-1_aarch64.deb
  ```

---

## Usage

After installation, start Netpulse as usual:

```sh
netpulse
```

---

## Notes

- If you are using a 32-bit OS, use the `armv7-unknown-linux-gnueabihf` target and the corresponding Speedtest CLI download.
- For more details, see the [full documentation](docs/).

---

## Browser Compatibility & Security Notes

NetPulse uses modern web technologies (Chart.js, jsPDF, HTML5 canvas) for chart rendering and export.  
**Some privacy-focused browsers (such as LibreWolf or hardened Firefox) may block chart display or export features** due to strict security settings, especially when accessing the dashboard over HTTP (not HTTPS).

**If you experience issues such as:**
- The chart not displaying
- Export Image/PDF not working
- Console errors about "Blocked from extracting canvas data"

**Try the following:**
- Use a mainstream browser like Chromium or Chrome for best compatibility
- Access the dashboard over HTTPS if possible
- Ensure all resources are loaded from the same origin (no mixed content)
- If using LibreWolf or hardened Firefox, consider relaxing certain privacy/security settings for your NetPulse dashboard

For more information, see:
- [MDN: CORS enabled image](https://developer.mozilla.org/en-US/docs/Web/HTML/CORS_enabled_image)
- [Chart.js Browser Support](https://www.chartjs.org/docs/latest/getting-started/installation.html#browser-support)