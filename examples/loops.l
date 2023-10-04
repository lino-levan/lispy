(var x 0)
(while (< x 10)
	(print "[count] " x)
	(var x (+ x 1))
)
(var first 0)
(var second 1)
(while (< first 610)
	(print "[fib] " first)
	(var temp (+ first second))
	(var first second)
	(var second temp)
)
(for (var x 0) (< x 10) (var x (+ x 1))
	(print
		"hi for a for loop. we are at increment "
		x
	)
)
