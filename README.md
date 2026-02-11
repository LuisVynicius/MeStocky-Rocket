# Mestocky

This repository contains the backend of the Mestocky project, developed to serve as the main API for the inventory system used by the Angular frontend (available on this same profile).

## Environment Variables

- DATABASE_URL: Database connection URL.
- JWT_SECRET: Key used for generating and validating JWT tokens.
- ENCRYPT_COST: Password encryption cost (between 4 and 31).
// Higher values increase security but make the hashing process slower.

## Routes
Below is an overview of the available routes, grouped by entity.

- Categories:
 - GET:
  - /category -> Returns all categories.
  - /category/admin -> Returns categories with additional information (admins only).
 - POST:
  - /category -> Creates a new category.
 - PUT:
  - /category -> Updates an existing category.
 - DELETE:
  - /category/<id> -> Deletes a category by ID.

- Products:
 - GET:
  - /product -> Returns all products.
  - /product/informations -> Returns general stock data for products.
  - /product/<id> -> Returns a specific product.
 - POST:
  - /product -> Creates a new product.
 - PUT:
  - /product -> Updates a product.
  - /product/quantity -> Changes the quantity of a product, recording the reason.
 - DELETE:
  - /product/<id> -> Deletes a product by ID.

- Reasons:
 - GET:
  - /reason -> Returns all reasons.
 - POST:
  - /reason -> Creates a new reason.
 - PUT:
  - /reason -> Updates a reason.
 - DELETE:
  - /reason/<id> -> Deletes a reason by ID.

- Reports:
 - GET:
  - /report -> Returns all movement reports.
 - PUT:
  - /report -> Updates a report.

- Users:
 - GET:
  - /user -> Returns all users.
  - /login/valid -> Checks if the current token is valid.
 - POST:
  - /user -> Creates a new user.
  - /login -> Generates an authentication token.
 - PUT:
  - /user/informations -> AUpdates basic information of the logged-in user.
  - /user/credentials -> Updates the credentials of the logged-in user.
 - DELETE:
  - /user/<id> -> Deletes a user by ID.