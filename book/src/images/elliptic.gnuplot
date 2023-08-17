#set terminal wxt size 350,262 enhanced font 'Verdana,10' persist

set zeroaxis
set samples 10000

# Line styles
set style line 1 linecolor rgb '#0060ad' linetype 1 linewidth 1
set xrange[-2:2]
set yrange[-2:2]

a = 0
b = 1
#f(x) = a * sin(x)
#f(x) = sqrt(x**3 + a*x + b), -sqrt(x**3 + a*x + b)
f(x) = sqrt(x**3 + a*x + b)
plot f(x) title 'sqrt256k' with lines linestyle 1, \
  -f(x) notitle with lines linestyle 1
