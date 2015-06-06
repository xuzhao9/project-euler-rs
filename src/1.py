import sys

class NumberTheory():
    def __init__(self, n):
        self.primes = self.__init_primes(n)
        self.factorize_cache = {}
        print('Prime count =>', len(self.primes))

    def __init_primes(self, n):
        visited = [False for _ in range(n + 1)]
        visited[0] = visited[1] = True
        for i in range(2, n + 1):
            if visited[i]:
                continue
            for j in range(i*i, n + 1, i):
                visited[j] = True
        return [i for i in range(2, n + 1) if not visited[i]]

    def factorize(self, n):
        if n not in self.factorize_cache:
            result = []
            d = n
            for prime in self.primes:
                if prime * prime > d:
                    break
                e = 0
                while d % prime == 0:
                    e += 1
                    d = d // prime
                if e > 0:
                    result.append((prime, e))
            if d > 1:
                result.append((d, 1))
            if not result:
                result = [(1, 0)]
            self.factorize_cache[n] = result
        return self.factorize_cache[n]

class Problem():
    def __init__(self):
        self.number_theory = None

    def solve(self):
        #assert(self.get(100) == 1035)
        print(self.get(10**8))

    def get(self, n):
        self.number_theory = NumberTheory(n)
        squares = self.__get_squares()
        complements = self.__get_complements()
        triple_sum = 0
        for key in complements:
            complement = complements[key]
            c_len = len(complement)
            for i in range(c_len):
                a = complement[i]
                for j in range(i + 1, c_len):
                    c = complement[j]
                    x = (a + 1) * (c + 1)
                    if x in squares:
                        b = squares[x]
                        triple_sum += a + b + c
        return triple_sum

    def __get_squares(self):
        result = {}
        for p in self.number_theory.primes:
            result[(p + 1)**2] = p
        return result

    def __get_complements(self):
        result = {}
        prime_count = len(self.number_theory.primes)
        for i in range(prime_count):
            p = self.number_theory.primes[i]
            factorization = self.number_theory.factorize(p + 1)
            complement = 1
            for a, e in factorization:
                if e % 2 != 0:
                    complement *= a
            if complement not in result:
                result[complement] = []
            result[complement].append(p)
            if i % 10000 == 0:
                print(i, 'prime =>', p, factorization)
        return result

def main():
    problem = Problem()
    problem.solve()

if __name__ == '__main__':
    sys.exit(main())
