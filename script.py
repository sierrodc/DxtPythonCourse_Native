from myrustlib import MyType, MyStruct
import threading
import time

x = MyType(10)
fibo = x.get_fibonacci()
print(fibo)
str = x.get_my_struct()
print(f"Values: {str.number1} {str.number2}")

class MyOracleThread(threading.Thread):
    def __init__(self, delay: int):
        threading.Thread.__init__(self)
        self.delay = delay
        self.result = None
    
    def run(self):
        mt = MyType(self.delay)
        #self.result = x.wait(int(self.delay/1000))
        self.result = x.wait_mt(int(self.delay/1000))


thread1 = MyOracleThread(6000)
thread2 = MyOracleThread(8000)

start_time = time.time()
# Start new Threads
thread1.start()
thread2.start()

thread1.join()
thread2.join()

print(f"Result: ({ thread1.result }, { thread2.result } ). Overall executed in: {time.time() - start_time}")