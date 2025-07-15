# Rust Programming Projects Report
### Mercatura Forum Blockchain Development Team

**Developer:** Abdelrahman Emad  
**Position:** Junior Blockchain Developer  
**Department:** Blockchain Development  
**Date:** July 15 2025  
**Repository:** [github.com/Aboodtt404/Merc-Tasks](https://github.com/Aboodtt404/Merc-Tasks)

## Project Overview
This report details the work completed on the final project, the "Rusty Store Inventory Management System" (RuStock). This command-line application is designed to manage a store's inventory, sales, and supply chain, demonstrating a practical application of Rust for building robust and efficient systems.

## Task Implementations

### Final Task: Rusty Store Inventory Management System (RuStock)
**Objective:** Develop a comprehensive, command-line-based inventory management system.

**Implementation Details:**
- **Inventory Management:** Core functionality to add, edit, delete, and view products in the store's inventory. Each product has a unique ID, name, description, price, and quantity.

- **Sales Management:** A module for recording sales transactions. It updates product quantities upon sale and maintains a history of all sales.

- **Supply Management:** Functionality for recording purchases of new or existing stock, updating inventory levels accordingly, and maintaining a purchase history.

- **Reporting System:** Generation of user-friendly, text-based reports for inventory, sales, and purchase history. The system checks if there is data to report before generating a file.

- **Database:** SQLite is used for persistent data storage of products, sales, and purchases.

- **Code Structure:** The project is organized into modules for products, sales, purchases, and database interactions, promoting a clean and maintainable codebase.

## Rust Topics Studied

The following Rust concepts were studied from Rust's official book and other resources:

1. Smart Pointers
2. Fearless Concurrency
3. Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
4. Object Oriented Programming Features of Rust 