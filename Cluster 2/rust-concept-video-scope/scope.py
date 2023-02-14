# Python scope for Rust concept video

# Python is interpreted line by line at run time, starting from line 1 and ending at the last line of code.
# If a variable is not yet defined, then it can not be used, and if you try to then the program will error.

# y = 11

try:
    print(y)  # this line should error as x is not defined outside the function the_answer()
except NameError:
    print(
        "y is not yet defined in our script, and therefore can not be called without handling its error. In Rust, this would be ok, as the order in which things are defined \n are not important, just that they are all used. In Python, the order in which things are defined is important, as each line is executed sequentially. \n")


########################################################################################################################

# Local scope within a function -- x is defined within the the_answer() function, and can not be used outside of it.
def the_answer():
    x = 42  # the scope of this x variable is contained only within the function the_answer()
    print("The answer is " + str(x) + "\n")


# call the_answer() function
the_answer()  # this line should not error, and should print the statement defined in the_answer() function

# we are going to try and print x using error handling. (We know it won't work already, and the error message will be displayed)
try:
    print(x)  # this line should error as x is not defined outside the function the_answer()
except NameError:
    print(
        " x is not defined in this scope so we can not print the value of x, because none exists outside of the prior function. \n")

x = 23  # run time will accept this x variable being defined in the program, even though it is not being used, which Cargo in Rust will not accept unless explicitly told to do so.
# RUST WOULD ALLOW THIS, BUT NOT PYTHON
print(x)

########################################################################################################################

y = 11  # here we define a new global variable called y and set it to 11

print("the variable y is now defined in our script so we can print the value of y which is " + str(
    y) + "\n")  # should print 11


def print_y():
    global y  # here we create a new global variable within the function, which would otherwise have been a local variable, but not it is useable outisde of the function
    y = 12  # here we redefine the global value of the variable y to 12
    print(y)


print_y()  # should print 12
print("the variable y is now defined in our script so we can print the value of y which is " + str(
    y))  # should still print 12
