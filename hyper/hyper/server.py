# from typing import Union

# from fastapi import FastAPI

# server = FastAPI()


# @server.get("/")
# def read_root():
#     return { "Allahu": "Akbar" }


# @server.get("/items/{item_id}")
# def read_item(item_id: int, q: Union[str, None] = None):
#     return {"item_id": item_id, "q": q}

import python_wrapper;

output = python_wrapper.sum_as_string(6, 9)

print('Output: ', output)