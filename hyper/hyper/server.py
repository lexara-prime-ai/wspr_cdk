import asyncio

import python_wrapper.python_wrapper


async def main():
    output = await python_wrapper.python_wrapper.get_wspr_spots("10", "JSON")
    print("Output: ", output)


if __name__ == "__main__":
    asyncio.run(main())
