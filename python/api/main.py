import requests

#source: https://realpython.com/python-api/

"""
API stands for Application Programming Interface

API acts as a communaction layer, that allows different systems to talk to each other without
having to understand exaclty what each other does.
"""

# basic request and acess of response text content
response = requests.get("https://randomuser.me/api/")
print(response.text)

"""
Request and Response

- Requests: contain relevant data regarding your API request call, such as the base
URL, the endpoint, the method used, the headers, and so on.

- Responses: contain relevant data returned by the server, including the data or content,
the status code, and the headers.
"""

response = requests.get("https://api.thedogapi.com/v1/breeds")
print(response)
print(response.request)
print(response.text)
print(response.status_code)
print(response.headers)

request = response.request
print(request.url)
print(request.path_url)
print(request.method)
print(request.headers)

"""
Status Code

The status code will inform if your request was successfull

| Status code               | Description                                                                   |
| ------------------------- | ----------------------------------------------------------------------------- |
| 200 OK                    | Your request was successful!                                                  |
| 201 Created               | Your request was accepted and the resource was created.                       |
| 400 Bad Request           | Your request is either wrong or missing some information.                     |
| 401 Unauthorized          | Your request requires some additional permissions.                            |
| 404 Not Found             | The requested resource does not exist.                                        |
| 405 Method Not Allowed    | The endpoint does not allow for that specific HTTP method.                    |
| 500 Internal Server Error | Your request wasnt expected and probably broke something on the server side.  |
"""

response = requests.get("https://api.thedogapi.com/v1/breedz")
print(response)
print(response.status_code)
print(response.reason)

#difference between content and response
response = requests.get("https://api.thedogapi.com/v1/breeds/1")
print(response.text)
print(response.content)
print(response.json())
print(response.json()["name"])

"""
HTTP Methods

When calling and API, there are a few different methods that you can use to specify what actions
you want to execute

| HTTP Method | Description                  | Requests method   |
| ----------- | ---------------------------- | ----------------- |
| POST        | Create a new resource.       | requests.post()   |
| GET         | Read an existing resource.   | requests.get()    |
| PUT         | Update an existing resource. | requests.put()    |
| DELETE      | Delete an existing resource. | requests.delete() |

These four methods are typically referred to as CRUD operations, as they allow you to Create, Read,
Update and Delete resources.
"""

print(requests.post("https://api.thedogapi.com/v1/breeds/1"))
print(requests.get("https://api.thedogapi.com/v1/breeds/1"))
print(requests.put("https://api.thedogapi.com/v1/breeds/1"))
print(requests.delete("https://api.thedogapi.com/v1/breeds/1"))

#The post, put and delete returns 405 as status code, that means that we dont are allowed to perform it.

"""
Query Parameters

Query is a way to filter the api response. To add query paramters, you need to add a question mark '?' at
the end of api URL. If you want multiple query in your request, you can use a & to pass them.
"""

print(requests.get("https://randomuser.me/api/").json())
print(requests.get("https://randomuser.me/api/?gender=female").json())
print(requests.get("https://randomuser.me/api/?gender=female&nat=de").json())