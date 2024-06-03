import asyncio

import python_wrapper.python_wrapper


async def main():
    output = await python_wrapper.python_wrapper.get_wspr_spots("1", "JSON")
    print("Output: ", output.get_data())


if __name__ == "__main__":
    asyncio.run(main())
