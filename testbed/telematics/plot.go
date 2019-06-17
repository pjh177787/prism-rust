package main

import (
	"bufio"
	"fmt"
	"github.com/ziutek/rrd"
	"os"
	"strings"
	"time"
	"path"
)

func plot(nodesFile, dataDir, content, node, output string, step uint) {
	nodes := make(map[string]string)
	file, err := os.Open(nodesFile)
	if err != nil {
		fmt.Println("Error opening node list:", err)
		os.Exit(1)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		s := strings.Split(scanner.Text(), ",")
		name := s[0]
		p := path.Clean(dataDir + "/" + name + ".rrd")
		nodes[name] = p
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading node list:", err)
		os.Exit(1)
	}
	if _, ok := nodes[node]; !ok {
		fmt.Println("Node", node, "does not exist")
		os.Exit(1)
	}
	g := rrd.NewGrapher()
	g.SetSize(800, 300)
	switch content {
	case "txrate":
		// create def for each node, and create a cdef to sum them up (for generate) and get max/min (for confirm)
		genSum := ""
		nodeConfirmSet := ""
		for n, p := range nodes {
			g.Def(n + "_gen", p, "generated_tx", "AVERAGE", fmt.Sprintf("step=%v", step))
			g.Def(n + "_confirm", p, "confirmed_tx", "AVERAGE", fmt.Sprintf("step=%v", step))
			if genSum == "" {
				genSum = n + "_gen"
			} else {
				genSum += "," + n + "_gen,+"
			}
			nodeConfirmSet += n + "_confirm,"
		}
		g.CDef("gen_sum", genSum)
		g.CDef("confirm_max", fmt.Sprintf("%s%v,SMAX", nodeConfirmSet, len(nodes)))
		g.CDef("confirm_min", fmt.Sprintf("%s%v,SMIN", nodeConfirmSet, len(nodes)))
		g.CDef("confirm_avg", fmt.Sprintf("%s%v,AVG", nodeConfirmSet, len(nodes)))
		g.CDef("min_max_diff", "confirm_max,confirm_min,-")
		// plot the lines
		g.Line(1.0, "gen_sum", "00FF00", "Generated")
		g.Line(1.0, node + "_confirm", "FF0000", node + " confirmed")
		g.Line(1.0, "confirm_min", "")
		g.Area("min_max_diff", "0000FF15", "STACK")	// this area is stacked on confirm_min, so we should sub min
		g.Line(1.0, "confirm_avg", "0000FF", "Avg confirmed")
	case "blockdelay":
		g.Def(node + "_proposer_delay", nodes[node], "proposer_delay_mean", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Def(node + "_voter_delay", nodes[node], "voter_delay_mean", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Def(node + "_tx_delay", nodes[node], "tx_delay_mean", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Line(1.0, node + "_proposer_delay", "FF0000")
		g.Line(1.0, node + "_voter_delay", "00FF00")
		g.Line(1.0, node + "_tx_delay", "0000FF")
	case "queue":
		g.Def(node + "_queue", nodes[node], "queue_length", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Line(1.0, node + "_queue", "0000FF")
	case "mining":
		g.Def(node + "_mined_proposer", nodes[node], "mined_proposer", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Def(node + "_mined_voter", nodes[node], "mined_voter", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Def(node + "_mined_transaction", nodes[node], "mined_transaction", "AVERAGE", fmt.Sprintf("step=%v", step))
		g.Line(1.0, node + "_mined_proposer", "FF0000")
		g.Line(1.0, node + "_mined_voter", "00FF00")
		g.Line(1.0, node + "_mined_transaction", "0000FF")
	default:
		fmt.Println("Plot content options: txrate, blockdelay, queue, mining")
		os.Exit(1)
	}
	_, e := g.SaveGraph(output, time.Now().Add(-time.Duration(600) * time.Second), time.Now())
	if e != nil {
		fmt.Println("Error plotting data:", e)
	}
}

