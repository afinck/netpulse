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