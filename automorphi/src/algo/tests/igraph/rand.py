import igraph
import random

f = open("./rand.rs", "w")

f.write("use crate::Graph;\n\n")
global_counter = 0

for _ in range(100):
	while True:
		global_counter += 1
		random.seed(global_counter)

		g = igraph.Graph()
		n = random.randint(5, 25)
		g.add_vertices(n)

		RandEdges = []
		for x in range(random.randint(n, n*(n-1)//2)):
			e = [0,0]
			while e[0] == e[1]:
				e = random.sample(range(0,g.vcount()), 2)
			RandEdges.append(e)
		g.add_edges(RandEdges)
		g = g.simplify()

		if g.count_isomorphisms_vf2() > 1:
			continue

		S = "#[test]\nfn igraph_rand_" + str(global_counter) + "_is_asymmetric() {\n"
		S += "\tlet mut g = Graph::empty(" + str(g.vcount()) + ");\n\n"
		M = g.get_adjacency()
		for i in range(g.vcount()):
			for j in range(i+1, g.vcount()):
				if M[i][j] > 0:
					S += "\tg.set_edge(" + str(i) + ", " + str(j) + ", true);\n"
		S += "\n"
		S += "\tassert_eq!(g.search_asymmetry(), true);\n"
		S += "}\n"
		f.write(S)
		break
