# MEMOserv Usage Guide

MEMOserv is a database server that uses the memodb database engine. Here are the basic operations you can perform using this server over HTTP.

## Create a collection

To create a new collection, make a POST request to the path /collection_name. If the collection is created successfully, you will receive an HTTP 201 (Created) status. In case the collection already exists, you will receive an HTTP 304 (Not Modified) status.

```http
GET http://localhost:3000/usuarios
```

## Add a document to a collection

To add a new document to a collection, make a POST request to the /collection_name path. You must include the document as JSON in the body of the request. If the document is successfully added, you will receive an HTTP 201 (Created) status along with the ID of the new document.

```http
POST http://localhost:3000/usuarios
Content-Type: application/json

{
  “first name”: “John”,
  “last name”: “Perez”,
  “age”: 30
}
```

## Get the list of all collections

To get a list of all collections, make a GET request to the path /. You will receive an HTTP 200 (OK) status along with the list of collections.

```http
GET http://localhost:3000/
```


## Get all documents in a collection

To get all documents in a specific collection, make a GET request to the path /collection_name/all. You will receive an HTTP 200 (OK) status along with a list of all documents in the collection.
  
```http
GET http://localhost:3000/usuarios/all
```

## Get a document by ID

To get a specific document by its ID, make a GET request to the path /collection_name/id. You will receive an HTTP 200 (OK) status along with the requested document. In case the document is not found, you will receive an HTTP 404 (Not Found) status.

```http
GET http://localhost:3000/usuarios/1
```

## Search for documents in a collection

To search for documents in a collection based on certain criteria, make a GET request to the path /collection_name/find. You must include the search criteria as query parameters in the URL.

```http
GET http://localhost:3000/usuarios/find?nombre=Juan
```

## Delete a collection

To delete a collection, make a DELETE request to the /collection_name path. Be sure to include an “amisure” header with the value “yes” to confirm the deletion. You will receive an HTTP 200 (OK) status if the collection is successfully deleted.

```http
DELETE http://localhost:3000/usuarios
amisure: yes
```


## Delete a document

To delete a specific document, make a DELETE request to the /collection_name/id path. You will receive an HTTP 200 status (OK) if the document is successfully deleted.

> 
> **Note:** it is not necessary to include an “amisure” header to confirm the deletion of a document. It is advisable not to use it to avoid accidental deletion of collections in case of error when creating the request.
>


```http
DELETE http://localhost:3000/usuarios/1
```


____

Now you're ready to start using MEMOserv to manage your data efficiently over HTTP!
