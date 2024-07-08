import igraph
L = ["Bull", "Chvatal", "Coxeter", "Cubical", "Diamond", "Dodecahedral", "Folkman", "Franklin", "Frucht", "Grotzsch", "Heawood", "Herschel", "House", "HouseX", "Icosahedral", "Krackhardt_Kite", "Levi", "McGee", "Meredith", "Noperfectmatching", "Nonline", "Octahedral", "Petersen", "Robertson", "Smallestcyclicgroup", "Tetrahedral", "Thomassen", "Tutte", "Uniquely3colorable", "Walther", "Zachary"]

f = open("./famous.rs", "w")

f.write("use crate::Graph;\n\n")

for l in L:
	g = igraph._igraph.GraphBase.Famous(l)
	if not g.is_simple(): continue

	if l == "Meredith":
		symmetric = True
	else:
		symmetric = g.count_isomorphisms_vf2() > 1

	S = "#[test]\nfn igraph_"
	S += l.lower()
	S += "_is_"
	if not symmetric: S += "a"
	S += "symmetric() {\n"
	S += "\tlet mut g = Graph::empty(" + str(g.vcount()) + ");\n\n"
	M = g.get_adjacency()
	for i in range(g.vcount()):
		for j in range(i+1, g.vcount()):
			if M[i][j] > 0:
				S += "\tg.set_edge(" + str(i) + ", " + str(j) + ", true);\n"
	S += "\n"
	S += "\tassert_eq!(g.search_asymmetry(), " + str(not symmetric).lower() + ");\n"
	S += "}\n"
	f.write(S)
f.close()
