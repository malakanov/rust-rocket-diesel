
# Blog api by rust : rocket + diesel + sqlite

For project run

```
  diesel migration run
```
```
  cargo run
```

## API Reference

#### Get all posts

```http
  GET /api/posts
```


#### Get post

```http
  GET /api/post/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `integer` | **Required**. Id of post to fetch |

#### Create post

```http
  POST /api/post
```
Body
| Key        | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `title`      | `string` | **Required**. |
| `body`      | `string` | **Required**. |
| `genre`      | `string` | **Required**.|
