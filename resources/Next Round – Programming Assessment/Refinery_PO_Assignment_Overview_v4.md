Refinery Purchase Order System – Assignment
Overview
Document date: 2026-02-12
Overview
This take-home package contains two assignments: a Front-End track and a Back-End track.
Candidates may choose ONE track based on their role focus. Candidates who wish to demonstrate
full-stack capability may choose to complete BOTH tracks.
Shared Domain Rules
• Single supplier per Purchase Order.
• PO lifecycle: Draft → Submitted → Approved/Rejected → Fulfilled.
• All status changes must append to a timeline/audit history.
• Item pricing and lead time must be snapshotted at PO submission.
Provided JSON Dataset
The file refinery_items_50_5suppliers_strict.json contains 50 refinery-related equipment items
distributed across 5 suppliers. Each item includes structured engineering and procurement data.
• Unique ID
• Name, category, manufacturer, supplier
• Model number
• Price (USD)
• Lead time (days)
• In-stock flag
• Structured specifications (key/value pairs)
• Optional compatibleWith references
Example JSON object structure:
{ "id": "VLV-0101", "name": "Ball Valve 4 in Class 300", "category": "Valve", "supplier":
"Flowserve", "manufacturer": "Flowserve", "model": "FLS-BV-4IN-300", "priceUsd": 1850,
"leadTimeDays": 21, "inStock": true, "specs": { "standard": "API 608", "pressureClass":
"ASME 300", "bodyMaterial": "ASTM A216 WCB" }, "compatibleWith": [ "GST-0003", "GST-0007" ]
}
Front-End candidates may load this file directly as mock data. Back-End candidates should design how
this structure would be stored, indexed, validated, and queried within a Catalog service.
Track Selection
• Front-End track: implement the Buyer workflow and UI.
• Back-End track: design the services, APIs, and database supporting the workflow.
• Optional: complete both tracks to demonstrate full-system capability.Full System Outcome
If both assignments are completed, they form a coherent refinery procurement system with a
React-based Buyer interface, structured service contracts, and persistent storage.