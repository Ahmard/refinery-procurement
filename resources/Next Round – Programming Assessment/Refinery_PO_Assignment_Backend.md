Back-End Assignment – Refinery Purchase Order
System
Document date: 2026-02-12
Context
Model the backend for the Refinery Purchase Order system using microservices and deliberate
database schema design.
Architecture Requirements
• Define service boundaries (Catalog + Procurement minimum).
• Document data ownership per service.
• Define synchronous APIs and optional async events.
• Describe idempotency and failure handling.
API Requirements
• OpenAPI specification required.
• Catalog: search/filter/sort endpoints.
• Procurement: draft creation, line management, submit PO, status transitions.
• Return 409 Conflict for supplier mismatch.
Database Requirements
• Explicit schema with keys, constraints, and indexes.
• Single-supplier enforcement at service + DB level.
• PO number generation strategy.
• Status timeline/audit table required.
Evaluation Focus
Service boundary clarity, schema quality, API usability, idempotency, lifecycle modeling, practicality.