# https://qiita.com/icoxfog417/items/07cbf5110ca82629aca0
import asyncio
from datetime import datetime

def now():
    dt = datetime.now()
    s_dt = dt.strftime('%H:%M:%S')
    msec = dt.microsecond // 1000
    return f"{s_dt}.{msec}"

Seconds = [
    ("first", 0),
    ("second", 1),
    ("third", 0.5),
    ("forth", 0.3),
    ("fifth", 1)
]

async def sleeping(order, seconds, hook=None):
    await asyncio.sleep(seconds)
    if hook:
        hook(order)
    return order

def main_01_basic_async():
    async def basic_async(label):
        for s in Seconds:
            r = await sleeping(*s)
            print(f"{label}{now()} {r} is finished")
        return True

    loop = asyncio.get_event_loop()
    #loop.run_until_complete(basic_async("A:"))
    for s in "ABC":
        asyncio.ensure_future(basic_async(f"{s}:"))
    loop.run_forever()

if __name__ == "__main__":
    main_01_basic_async()
