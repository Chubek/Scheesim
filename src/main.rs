use scheesim_lexparse::ScheesimSchema;

fn main() {   
    let ss = 
    r#"[Tab; 1]
+Node1<<in=(grnd); out=(myNode)>>
	MyVS<BT[grnd; myRes]>:
	 	VoltageSource<voltage=20mV, type=linear>
	myRes<BT[myVS; MyNode]>:
	 	Resistor<resistance=10ohm, type=linear>
+MyNode<<Node1; SessNode2; grnd>>
	myCap<BT[Node1; myCCCS]>:
		Capacitor<capacitance=1μF, type=dynamic>
	myCCCS<BT[myCap; tran4]>:
		CurrentControlledCurrentSource<current=2A, type=nonlinear>
	tran4<TT[grnd; myCap; myCCCS]>:
		BJT<power=0.2W, bias=NPN>
	(*) mySubMTModule<MT>:
		*InternalNode1<igrnd; InternalNode2>
			myInnerRes<BT[ext; myIntCap]>
				Resistor<ressitance=2ohm, type=linear>
			myIntInd<BT[igrnd; myIntNtNode]>
				Inductor<inductance=10MH, type=dynamic>
		*myIntNode<internalNode1, ext>
			myMM<TT[myIntNode; ext]
				VoltageControlledCurrentSource<voltage=10V; current=1A>"#;

    let schema = ScheesimSchema::from(ss.trim());

    println!("{:?}", schema);
}
