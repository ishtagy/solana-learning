markdown

# Inventory Management System

## Overview

The Inventory Management System is a simple command-line application written in Rust that allows users to manage products, record sales and purchases, and generate reports. The system provides functionalities for adding, editing, deleting, and listing products, as well as tracking sales and purchases.

## Features

- Add, edit, and delete products in the inventory.
- Record sales and purchases.
- Generate sales and purchase reports.
- Simple authentication mechanism.

## Prerequisites

- Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the official installation guide.

## Getting Started

### Clone the Repository

```bash
Clone repo
cd inventory-management-system
```

## Build the Project

Use the following command to build the project:

```bash
cargo build --release
```

## Run the Project

After building the project, run it with the following command:

```bash
cargo run
```

## Usage

Upon running the application, users will be prompted to enter a password for authentication. Once authenticated, the main menu will appear, allowing users to choose different options:

- Add Product
- Edit Product
- Remove Product
- List Products
- Record Sale
- Record Purchase
- Generate Report
- Exit

## Authentication

The application has a simple authentication system. The default password is set to 12345. Users will be prompted to enter this password before accessing the main menu.

Testing
To run the tests, execute the following command:

```bash
cargo test
```

Example Usage
Here are some basic examples of how to use the system:

## Adding a Product

1. Select option 1 from the menu.
2. Input the product name, description, price, and quantity when prompted.

## Recording a Sale

1. Select option 5 from the menu.
2. Input the index of the product, quantity sold, and sale price.

## Generating Reports

1. Select option 7 from the menu to view the current inventory, sales, and purchase reports.
