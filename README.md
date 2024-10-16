# Compiler in Rust - Language: Void

Overview

Void is a new language aimed at providing simple networking and database functionalities. Below is a detailed breakdown of features planned for inclusion, organized by Computer Networks (CN) and Database Management Systems (DBMS) curriculum units [1, 2, 3, 4]


----------------------------------------------------------------------------------------------------

Computer Networks (CN)

----------------------------------------------------------------------------------------------------

Unit 1: TCP/IP Protocol Suite

Feature: Implement Basic Socket Communication 
Include the ability to send and receive data over TCP and UDP.
Support for IP Addressing (IPv4) with basic error handling.
Allow simple client-server communication (request-response model).

Unit 2: Underlying Technologies (Ethernet and Wireless Technologies)

Feature: Wired and Wireless Network Connectivity
Enable support for Ethernet and Wi-Fi connections.
Simulate MAC address handling for devices connecting to a network.
Introduce network discovery tools (e.g., listing available networks, connecting to networks).

Unit 3: Network Layer Protocols

Feature: Support for IPv4/IPv6 Networking
Implement routing and NAT (Network Address Translation) to manage internal and external traffic.
Allow packet construction with headers (similar to IPv4/IPv6).
Include basic ICMP features for debugging (e.g., ping functionality).

Unit 4: Transport Layer Protocols

Feature: Advanced Networking Features
Implement TCP and UDP transport layer protocols with flow control (e.g., congestion control for TCP).
Support for multiple connections, including connection pooling.
Introduce data encryption in transit (e.g., TLS support).


----------------------------------------------------------------------------------------------------

Database Management Systems (DBMS)

----------------------------------------------------------------------------------------------------

Unit 1: Database Architecture & Introduction

Feature: Relational Database Support
Define a basic relational database system.
Implement a SQL-like query language for CRUD operations (Create, Read, Update, Delete).
Include support for connecting to external databases (e.g., PostgreSQL, MySQL).
Ability to define tables and schemas within the Void environment.

Unit 2: SQL Queries

Feature: Advanced SQL Handling
Support for complex SQL queries such as JOINs, nested queries, and transactions.
Implement basic query optimization (e.g., indexing)
Allow the Void compiler to manage connection states and handle query errors.

Unit 3: Database Transactions and Concurrency

Feature: Transaction Management
Support transaction isolation levels and locking mechanisms (e.g., optimistic/pessimistic locking).
Implement ACID properties for data integrity.
Handle rollback and commit features programmatically.

Unit 4: Advanced Database Topics

Feature: Scaling and Optimization
Implement connection pooling and support for distributed databases
Introduce replication (master-slave) and partitioning.
Include caching mechanisms for frequently queried data.
Allow database backup and recovery within the Void environment.

Summary

These features provide a solid foundation for the Void language's capabilities in networking and database management, ensuring compatibility, ease of use, and scalability for future projects. Let me know if there are specific parts you'd like to refine or add further details to!
