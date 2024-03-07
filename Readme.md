# Guía de Uso de MEMOserv

MEMOserv es un servidor de base de datos que utiliza el motor de base de datos memodb. A continuación, se detallan las operaciones básicas que puedes realizar utilizando este servidor a través de HTTP.

## Crear una colección

Para crear una nueva colección, realiza una solicitud POST a la ruta /collection_name. Si la colección se crea con éxito, recibirás un estado HTTP 201 (Created). En caso de que la colección ya exista, recibirás un estado HTTP 304 (Not Modified).

```http
GET http://localhost:3000/usuarios
```

## Agregar un documento a una colección

Para agregar un nuevo documento a una colección, realiza una solicitud POST a la ruta /collection_name. Debes incluir el documento como JSON en el cuerpo de la solicitud. Si el documento se agrega correctamente, recibirás un estado HTTP 201 (Created) junto con el ID del nuevo documento.

```http
POST http://localhost:3000/usuarios
Content-Type: application/json

{
  "nombre": "Juan",
  "apellido": "Pérez",
  "edad": 30
}
```

## Obtener la lista de todas las colecciones

Para obtener una lista de todas las colecciones, realiza una solicitud GET a la ruta /. Recibirás un estado HTTP 200 (OK) junto con la lista de colecciones.

```http
GET http://localhost:3000/
```


## Obtener todos los documentos de una colección

Para obtener todos los documentos de una colección específica, realiza una solicitud GET a la ruta /collection_name/all. Recibirás un estado HTTP 200 (OK) junto con una lista de todos los documentos en la colección.
  
```http
GET http://localhost:3000/usuarios/all
```

## Obtener un documento por ID

Para obtener un documento específico por su ID, realiza una solicitud GET a la ruta /collection_name/id. Recibirás un estado HTTP 200 (OK) junto con el documento solicitado. En caso de que el documento no se encuentre, recibirás un estado HTTP 404 (Not Found).

```http
GET http://localhost:3000/usuarios/1
```

## Buscar documentos en una colección

Para buscar documentos en una colección basándote en ciertos criterios, realiza una solicitud GET a la ruta /collection_name/find. Debes incluir los criterios de búsqueda como parámetros de consulta en la URL.

```http
GET http://localhost:3000/usuarios/find?nombre=Juan
```

## Eliminar una colección

Para eliminar una colección, realiza una solicitud DELETE a la ruta /collection_name. Asegúrate de incluir un encabezado "amisure" con el valor "yes" para confirmar la eliminación. Recibirás un estado HTTP 200 (OK) si la colección se elimina con éxito.

```http
DELETE http://localhost:3000/usuarios
amisure: yes
```

## Eliminar un documento

Para eliminar un documento específico, realiza una solicitud DELETE a la ruta /collection_name/id. Recibirás un estado HTTP 200 (OK) si el documento se elimina correctamente.

> 
> **Nota:** no es necesario incluir un encabezado "amisure" para confirmar la eliminación de un documento. Es aconsejable no usarlo para evitar la eliminación accidental de colecciones en caso de error al crear la solicitud.
>


```http
DELETE http://localhost:3000/usuarios/1
```



----
____

¡Ahora estás listo para comenzar a utilizar MEMOserv para gestionar tus datos de manera eficiente a través de HTTP!