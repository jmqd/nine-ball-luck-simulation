#!/usr/bin/env sh

# Data file
datafile="set-match-data.txt"

# Output file
outputfile="set-match-plot.png"

# Gnuplot script
gnuplot << EOF
set terminal pngcairo enhanced font 'Verdana,12'
set datafile separator "\t"
set key autotitle columnhead
set key noenhanced
set output '$outputfile'
set yrange [-1:101]
set xrange [0:55]
set title '[Set Match] Winning Percentage vs Luck Chance'
set grid
plot '$datafile' using 1:2 with lines linestyle 1, \
     '' using 1:2 with points linestyle 2 notitle, \
     '' using (\$1+1):2:(sprintf("%d%%", \$2)) with labels offset 1,0 notitle
EOF

echo "Plot generated: $outputfile"
