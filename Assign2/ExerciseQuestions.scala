import scala.io.StdIn.readLine
import scala.collection.mutable.ArrayBuffer
import scala.math._
import java.util.Calendar
import java.util.GregorianCalendar
import java.text.SimpleDateFormat

object ExerciseQuestions {
    def main(args: Array[String]) = {
        var i = 0
        do {
            println("Which function would you like to test out?")
            println("(1) divisors")
            println("(2) primes")
            println("(3) join")
            println("(4) pythagorean")
            println("(5) mergesort")
            println("(6) isPrimeDay")
            println("(7) isFriday")
            println("Enter a number, or 0 to quit")
            i = readLine.toInt
            i match {
                case 1 => divisorsPrompt()
                case 2 => primesPrompt()
                case 3 => joinPrompt()
                case 4 => pythagoreanPrompt()
                case 5 => mergesortPrompt()
                case 6 => isPrimeDayPrompt()
                case 7 => isFridayPrompt()
                case _ => 
            }
        } while (i != 0)
    }

    def divisorsPrompt() = {
        print("Enter the number to compute the divisors of: ")
        var n = readLine.toInt
        
        var divs = divisors(n)
        
        print("[")
        if(divs.length != 0) {
            for(i <- 0 until divs.length-1){
                print(divs(i))
                print(", ")
            }
            print(divs(divs.length-1))
        }
        print("]\n")
    }

    def divisors(n: Int) : Array[Int] = {
        var i = 0

        var divs = for { i <- 2 to (n/2) 
            if (n % i == 0)
            } yield i

        return divs.toArray
    }

    def primesPrompt() = {
        print("Enter the number to calculate primes up until: ")
        var n = readLine.toInt

        var prime = primes(n)

        print("[")
        if(prime.length != 0) {
            for(i <- 0 until prime.length-1){
                print(prime(i))
                print(", ")
            }
            print(prime(prime.length-1))
        }
        print("]\n")
    }

    def primes(n: Int) : Array[Int] = {
        var i = 0 

        val prime = for { i <- 2 to n
            if (divisors(i).length == 0)
            } yield i 
        
        return prime.toArray
    }

    def joinPrompt() = {
        print("Enter the seperator: ")
        var sep = readLine
        var lst = ArrayBuffer[String]()
        var listItem = " "
        print("Enter your list items one at a time\nEnter a space to stop\n")
        do {
            listItem = readLine
            if(listItem != " "){
                lst += listItem
            }
        } while (listItem != " ")

        var joined = join(sep, lst)

        print("\"")
        if(joined.length != 0) {
            for(i <- 0 until joined.length-1){
                print(joined(i))
            }
            print(joined(joined.length-1))
        }
        print("\"\n")
    }

    def join(sep: String, lst: ArrayBuffer[String]) : ArrayBuffer[String] = {
        if(lst.length == 0){
            return lst
        }

        var newList = ArrayBuffer[String]()
            
        for(i <- 0 until lst.length-1){
            newList += lst(i)
            newList += sep
        }
        newList += lst(lst.length-1)
        return newList
    }

    def pythagoreanPrompt() = {
        print("Enter the number to calculate pythagorean triples up until: ")
        var n = readLine.toInt

        var pyth = pythagorean(n)

        print("[")
        if(pyth.length != 0) {
            for(i <- 0 until pyth.length-1){
                print(pyth(i).toString)
                print(",")
            }
            print(pyth(pyth.length-1))
        }
        print("]\n")
    }

    def pythagorean(n: Int) : Array[(Int, Int, Int)] = {
        var a = 0
        var b = 0 
        var c = 0

        val pyth = for { c <- 1 to n 
            b <- 1 to n-1
            a <- 1 to n-1
            if (pow(a,2) + pow(b,2) == pow(c,2))
            if (a < b)
            } yield (a,b,c)

        return pyth.toArray
    }

    def mergesortPrompt() = {
        println("Enter the list of integers to be sorted, one element at a time")
        println("Enter a space to stop")
        var lst = ArrayBuffer[Int]()
        var listItem = " "
        do {
            listItem = readLine
            if(listItem != " "){
                lst += listItem.toInt
            }
        } while (listItem != " ")

        var merged = mergesort(lst)

        print("[")
        if(merged.length != 0) {
            for(i <- 0 until merged.length-1){
                print(merged(i))
                print(",")
            }
            print(merged(merged.length-1))
        }
        print("]\n")
    }

    def merge(lst1: ArrayBuffer[Int], lst2: ArrayBuffer[Int]) : ArrayBuffer[Int] = {
        if(lst1.length < 1){
            return lst2
        }
        if(lst2.length < 1){
            return lst1
        }
        var sortedItem = ArrayBuffer[Int]()
        if(lst1(0) < lst2(0)){
            sortedItem += lst1(0)
            lst1 -= (sortedItem(0))
            return sortedItem ++ merge(lst1, lst2)
        } else {
            sortedItem += lst2(0)
            lst2 -= (sortedItem(0))
            return sortedItem ++ merge(lst1, lst2)
        }
    }

    def mergesort(lst: ArrayBuffer[Int]) : ArrayBuffer[Int] = {
        if(lst.length < 1){
            return ArrayBuffer()
        }
        if(lst.length == 1){
            return lst
        }
        if(lst.length == 2){
            return merge(ArrayBuffer(lst(0)), ArrayBuffer(lst(1)))
        }
        var len = lst.length/2
        return merge(mergesort(lst.take(len)), mergesort(lst.takeRight(lst.length-len)))
    }

    def isFridayPrompt() = {
        print("Enter the year: ")
        var year = readLine.toInt
        print("Enter the month: ")
        var month = readLine.toInt - 1
        print("Enter the day: ")
        var day = readLine.toInt

        println(isFriday(new GregorianCalendar(year,month,day).getTime()))
    }

    def isFriday(date: java.util.Date) : Boolean = {
        var formatter = new SimpleDateFormat("u")
        return formatter.format(date) == "5"
    }

    def isPrimeDayPrompt() = {
        print("Enter the year: ")
        var year = readLine.toInt
        print("Enter the month: ")
        var month = readLine.toInt - 1
        print("Enter the day: ")
        var day = readLine.toInt

        println(isPrimeDay(new GregorianCalendar(year,month,day).getTime()))
    }

    def isPrimeDay(date: java.util.Date) : Boolean = {
        var formatter = new SimpleDateFormat("dd")
        return divisors(formatter.format(date).toInt).length == 0
    }
}