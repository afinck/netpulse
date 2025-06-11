# Netpulse Project

Netpulse is a backend application designed to handle measurement data and provide a web dashboard for visualization. It utilizes SQLite for data storage and serves a simple HTML/JS frontend for graphing using Chart.js. The application also supports PDF export functionality.

## Project Structure

```
netpulse
├── src
│   ├── main.rs               # Entry point of the application
│   ├── db.rs                 # Database connection and interactions
│   ├── measurements.rs        # Data structures and functions for measurements
│   ├── pdf_export.rs          # Functions for exporting data to PDF
│   ├── web
│   │   ├── mod.rs            # Module for web components
│   │   ├── routes.rs         # Application routes
│   │   └── handlers.rs       # Handler functions for routes
│   └── utils.rs              # Utility functions
├── static
│   ├── index.html            # Main HTML file for the dashboard
│   ├── app.js                # JavaScript for frontend interactions
│   └── chart.js              # Chart.js library for rendering graphs
├── templates
│   └── dashboard.html        # Template for the dashboard
├── migrations
│   └── 0001_create_tables.sql # SQL commands for creating database tables
├── Cargo.toml                # Rust project configuration
├── Cargo.lock                # Locked versions of dependencies
├── README.md                 # Project documentation
├── debian
│   ├── control               # Metadata for DEB package
│   ├── postinst              # Post-installation scripts for DEB package
│   └── rules                 # Build rules for DEB package
├── rpm
│   └── netpulse.spec         # Specifications for building RPM package
└── .gitignore                # Files and directories to ignore in Git
```

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone <repository-url>
   cd netpulse
   ```

2. **Install Dependencies**
   Ensure you have Rust and Cargo installed. Then, run:
   ```bash
   cargo build
   ```

3. **Database Setup**
   Run the SQL migration to set up the database:
   ```bash
   sqlite3 <database-file> < migrations/0001_create_tables.sql
   ```

4. **Run the Application**
   Start the server:
   ```bash
   cargo run
   ```

5. **Access the Dashboard**
   Open your browser and navigate to `http://localhost:3000` to view the dashboard.

## Features

- Measurement data handling and storage using SQLite.
- Web dashboard for data visualization using Chart.js.
- PDF export functionality for reports.
- DEB and RPM packaging support for easy installation.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.