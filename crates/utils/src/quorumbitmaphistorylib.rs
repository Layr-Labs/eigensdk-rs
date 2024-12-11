/**

Generated by the following Solidity interface...
```solidity
interface QuorumBitmapHistoryLib {}
```

...which was generated by the following JSON ABI:
```json
[]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod QuorumBitmapHistoryLib {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234601b5761078c90816100208239308160480152f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c806349ca6289146103b65780634fa878c314610388578063970e7a6f146101a35763982e7b8214610045575f80fd5b307f00000000000000000000000000000000000000000000000000000000000000001461019f57606036600319011261019f576004356044356001600160c01b038116906024359082810361019f57815f528360205260405f205480155f146100dc575050916100da925f5260205260405f206100c06104f0565b9163ffffffff431683525f602084015260408301526105e1565b005b825f949294528460205260405f20905f19810190811161018b576100ff9161058a565b509363ffffffff8554169263ffffffff43168094145f1461013d57505050506100da919067ffffffffffffffff82549181199060401b169116179055565b855467ffffffff000000001916602085901b67ffffffff0000000016179095556100da9492935090915f5260205260405f20906101786104f0565b9283525f602084015260408301526105e1565b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b608036600319011261019f5760443563ffffffff811680910361019f576024355f526004356020526101da60643560405f2061058a565b50906101e46104f0565b91549163ffffffff83168082526040602083019263ffffffff8660201c168452019360401c845282106102e9575163ffffffff1680159182156102df575b50501561023f57516040516001600160c01b039091168152602090f35b60405162461bcd60e51b815260206004820152606660248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d206265666f726520626c6f636b608482015265273ab6b132b960d11b60a482015260c490fd5b1090508280610222565b60405162461bcd60e51b815260206004820152606560248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d20616674657220626c6f636b4e6084820152643ab6b132b960d91b60a482015260c490fd5b604036600319011261019f5760206103a460243560043561059f565b6040516001600160c01b039091168152f35b606036600319011261019f5760243560043563ffffffff8216820361019f576044359067ffffffffffffffff821161019f573660238301121561019f5781600401359161040a6104058461054a565b610524565b926024602085838152019160051b8301019136831161019f57602401905b8282106104e05750505081519261044d6104446104058661054a565b9480865261054a565b602085019390601f19013685375f5b8151811015610496578061047d61047560019385610562565b518587610657565b63ffffffff61048c838a610562565b911690520161045c565b8486604051918291602083019060208452518091526040830191905f5b8181106104c1575050500390f35b825163ffffffff168452859450602093840193909201916001016104b3565b8135815260209182019101610428565b604051906060820182811067ffffffffffffffff82111761051057604052565b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761051057604052565b67ffffffffffffffff81116105105760051b60200190565b80518210156105765760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b8054821015610576575f5260205f2001905f90565b815f528060205260405f20549182155f146105bb575050505f90565b5f5260205260405f20905f19810190811161018b576105d99161058a565b505460401c90565b805468010000000000000000811015610510576106039160018201815561058a565b61064457815160208084015160409485015190941b67ffffffffffffffff191663ffffffff90921693901b67ffffffff000000001692909217919091179055565b634e487b7160e01b5f525f60045260245ffd5b9190815f528260205260405f2054925f5b8481106107005760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b80850385811161018b575f19810190811161018b5763ffffffff16845f528260205263ffffffff6107348260405f2061058a565b50541663ffffffff8516101561074d5750600101610668565b9450505050509056fea2646970667358221220ddbef2f44ded5414d464a876821c7b96c949fb411597ce0be9edca7e3efa0ddc64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x1BWa\x07\x8C\x90\x81a\0 \x8290\x81`H\x01R\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80cI\xCAb\x89\x14a\x03\xB6W\x80cO\xA8x\xC3\x14a\x03\x88W\x80c\x97\x0Ezo\x14a\x01\xA3Wc\x98.{\x82\x14a\0EW_\x80\xFD[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\x01\x9FW``6`\x03\x19\x01\x12a\x01\x9FW`\x045`D5`\x01`\x01`\xC0\x1B\x03\x81\x16\x90`$5\x90\x82\x81\x03a\x01\x9FW\x81_R\x83` R`@_ T\x80\x15_\x14a\0\xDCWPP\x91a\0\xDA\x92_R` R`@_ a\0\xC0a\x04\xF0V[\x91c\xFF\xFF\xFF\xFFC\x16\x83R_` \x84\x01R`@\x83\x01Ra\x05\xE1V[\0[\x82_\x94\x92\x94R\x84` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x01\x8BWa\0\xFF\x91a\x05\x8AV[P\x93c\xFF\xFF\xFF\xFF\x85T\x16\x92c\xFF\xFF\xFF\xFFC\x16\x80\x94\x14_\x14a\x01=WPPPPa\0\xDA\x91\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x85Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x95Ua\0\xDA\x94\x92\x93P\x90\x91_R` R`@_ \x90a\x01xa\x04\xF0V[\x92\x83R_` \x84\x01R`@\x83\x01Ra\x05\xE1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x80\xFD[`\x806`\x03\x19\x01\x12a\x01\x9FW`D5c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01\x9FW`$5_R`\x045` Ra\x01\xDA`d5`@_ a\x05\x8AV[P\x90a\x01\xE4a\x04\xF0V[\x91T\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x82R`@` \x83\x01\x92c\xFF\xFF\xFF\xFF\x86` \x1C\x16\x84R\x01\x93`@\x1C\x84R\x82\x10a\x02\xE9WQc\xFF\xFF\xFF\xFF\x16\x80\x15\x91\x82\x15a\x02\xDFW[PP\x15a\x02?WQ`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x10\x90P\x82\x80a\x02\"V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x90\xFD[`@6`\x03\x19\x01\x12a\x01\x9FW` a\x03\xA4`$5`\x045a\x05\x9FV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[``6`\x03\x19\x01\x12a\x01\x9FW`$5`\x045c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x9FW`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x9FW6`#\x83\x01\x12\x15a\x01\x9FW\x81`\x04\x015\x91a\x04\na\x04\x05\x84a\x05JV[a\x05$V[\x92`$` \x85\x83\x81R\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\x9FW`$\x01\x90[\x82\x82\x10a\x04\xE0WPPP\x81Q\x92a\x04Ma\x04Da\x04\x05\x86a\x05JV[\x94\x80\x86Ra\x05JV[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81Q\x81\x10\x15a\x04\x96W\x80a\x04}a\x04u`\x01\x93\x85a\x05bV[Q\x85\x87a\x06WV[c\xFF\xFF\xFF\xFFa\x04\x8C\x83\x8Aa\x05bV[\x91\x16\x90R\x01a\x04\\V[\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x04\xC1WPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\xB3V[\x815\x81R` \x91\x82\x01\x91\x01a\x04(V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\x10W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x05\x10W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x10W`\x05\x1B` \x01\x90V[\x80Q\x82\x10\x15a\x05vW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x05vW_R` _ \x01\x90_\x90V[\x81_R\x80` R`@_ T\x91\x82\x15_\x14a\x05\xBBWPPP_\x90V[_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x01\x8BWa\x05\xD9\x91a\x05\x8AV[PT`@\x1C\x90V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x05\x10Wa\x06\x03\x91`\x01\x82\x01\x81Ua\x05\x8AV[a\x06DW\x81Q` \x80\x84\x01Q`@\x94\x85\x01Q\x90\x94\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x93\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10a\x07\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a\x01\x8BW_\x19\x81\x01\x90\x81\x11a\x01\x8BWc\xFF\xFF\xFF\xFF\x16\x84_R\x82` Rc\xFF\xFF\xFF\xFFa\x074\x82`@_ a\x05\x8AV[PT\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a\x07MWP`\x01\x01a\x06hV[\x94PPPPP\x90V\xFE\xA2dipfsX\"\x12 \xDD\xBE\xF2\xF4M\xEDT\x14\xD4d\xA8v\x82\x1C{\x96\xC9I\xFBA\x15\x97\xCE\x0B\xE9\xED\xCA~>\xFA\r\xDCdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806349ca6289146103b65780634fa878c314610388578063970e7a6f146101a35763982e7b8214610045575f80fd5b307f00000000000000000000000000000000000000000000000000000000000000001461019f57606036600319011261019f576004356044356001600160c01b038116906024359082810361019f57815f528360205260405f205480155f146100dc575050916100da925f5260205260405f206100c06104f0565b9163ffffffff431683525f602084015260408301526105e1565b005b825f949294528460205260405f20905f19810190811161018b576100ff9161058a565b509363ffffffff8554169263ffffffff43168094145f1461013d57505050506100da919067ffffffffffffffff82549181199060401b169116179055565b855467ffffffff000000001916602085901b67ffffffff0000000016179095556100da9492935090915f5260205260405f20906101786104f0565b9283525f602084015260408301526105e1565b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b608036600319011261019f5760443563ffffffff811680910361019f576024355f526004356020526101da60643560405f2061058a565b50906101e46104f0565b91549163ffffffff83168082526040602083019263ffffffff8660201c168452019360401c845282106102e9575163ffffffff1680159182156102df575b50501561023f57516040516001600160c01b039091168152602090f35b60405162461bcd60e51b815260206004820152606660248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d206265666f726520626c6f636b608482015265273ab6b132b960d11b60a482015260c490fd5b1090508280610222565b60405162461bcd60e51b815260206004820152606560248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d20616674657220626c6f636b4e6084820152643ab6b132b960d91b60a482015260c490fd5b604036600319011261019f5760206103a460243560043561059f565b6040516001600160c01b039091168152f35b606036600319011261019f5760243560043563ffffffff8216820361019f576044359067ffffffffffffffff821161019f573660238301121561019f5781600401359161040a6104058461054a565b610524565b926024602085838152019160051b8301019136831161019f57602401905b8282106104e05750505081519261044d6104446104058661054a565b9480865261054a565b602085019390601f19013685375f5b8151811015610496578061047d61047560019385610562565b518587610657565b63ffffffff61048c838a610562565b911690520161045c565b8486604051918291602083019060208452518091526040830191905f5b8181106104c1575050500390f35b825163ffffffff168452859450602093840193909201916001016104b3565b8135815260209182019101610428565b604051906060820182811067ffffffffffffffff82111761051057604052565b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761051057604052565b67ffffffffffffffff81116105105760051b60200190565b80518210156105765760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b8054821015610576575f5260205f2001905f90565b815f528060205260405f20549182155f146105bb575050505f90565b5f5260205260405f20905f19810190811161018b576105d99161058a565b505460401c90565b805468010000000000000000811015610510576106039160018201815561058a565b61064457815160208084015160409485015190941b67ffffffffffffffff191663ffffffff90921693901b67ffffffff000000001692909217919091179055565b634e487b7160e01b5f525f60045260245ffd5b9190815f528260205260405f2054925f5b8481106107005760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b80850385811161018b575f19810190811161018b5763ffffffff16845f528260205263ffffffff6107348260405f2061058a565b50541663ffffffff8516101561074d5750600101610668565b9450505050509056fea2646970667358221220ddbef2f44ded5414d464a876821c7b96c949fb411597ce0be9edca7e3efa0ddc64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80cI\xCAb\x89\x14a\x03\xB6W\x80cO\xA8x\xC3\x14a\x03\x88W\x80c\x97\x0Ezo\x14a\x01\xA3Wc\x98.{\x82\x14a\0EW_\x80\xFD[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\x01\x9FW``6`\x03\x19\x01\x12a\x01\x9FW`\x045`D5`\x01`\x01`\xC0\x1B\x03\x81\x16\x90`$5\x90\x82\x81\x03a\x01\x9FW\x81_R\x83` R`@_ T\x80\x15_\x14a\0\xDCWPP\x91a\0\xDA\x92_R` R`@_ a\0\xC0a\x04\xF0V[\x91c\xFF\xFF\xFF\xFFC\x16\x83R_` \x84\x01R`@\x83\x01Ra\x05\xE1V[\0[\x82_\x94\x92\x94R\x84` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x01\x8BWa\0\xFF\x91a\x05\x8AV[P\x93c\xFF\xFF\xFF\xFF\x85T\x16\x92c\xFF\xFF\xFF\xFFC\x16\x80\x94\x14_\x14a\x01=WPPPPa\0\xDA\x91\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x85Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x95Ua\0\xDA\x94\x92\x93P\x90\x91_R` R`@_ \x90a\x01xa\x04\xF0V[\x92\x83R_` \x84\x01R`@\x83\x01Ra\x05\xE1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x80\xFD[`\x806`\x03\x19\x01\x12a\x01\x9FW`D5c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01\x9FW`$5_R`\x045` Ra\x01\xDA`d5`@_ a\x05\x8AV[P\x90a\x01\xE4a\x04\xF0V[\x91T\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x82R`@` \x83\x01\x92c\xFF\xFF\xFF\xFF\x86` \x1C\x16\x84R\x01\x93`@\x1C\x84R\x82\x10a\x02\xE9WQc\xFF\xFF\xFF\xFF\x16\x80\x15\x91\x82\x15a\x02\xDFW[PP\x15a\x02?WQ`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x10\x90P\x82\x80a\x02\"V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x90\xFD[`@6`\x03\x19\x01\x12a\x01\x9FW` a\x03\xA4`$5`\x045a\x05\x9FV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[``6`\x03\x19\x01\x12a\x01\x9FW`$5`\x045c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x9FW`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x9FW6`#\x83\x01\x12\x15a\x01\x9FW\x81`\x04\x015\x91a\x04\na\x04\x05\x84a\x05JV[a\x05$V[\x92`$` \x85\x83\x81R\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\x9FW`$\x01\x90[\x82\x82\x10a\x04\xE0WPPP\x81Q\x92a\x04Ma\x04Da\x04\x05\x86a\x05JV[\x94\x80\x86Ra\x05JV[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81Q\x81\x10\x15a\x04\x96W\x80a\x04}a\x04u`\x01\x93\x85a\x05bV[Q\x85\x87a\x06WV[c\xFF\xFF\xFF\xFFa\x04\x8C\x83\x8Aa\x05bV[\x91\x16\x90R\x01a\x04\\V[\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x04\xC1WPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04\xB3V[\x815\x81R` \x91\x82\x01\x91\x01a\x04(V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05\x10W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x05\x10W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x10W`\x05\x1B` \x01\x90V[\x80Q\x82\x10\x15a\x05vW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x05vW_R` _ \x01\x90_\x90V[\x81_R\x80` R`@_ T\x91\x82\x15_\x14a\x05\xBBWPPP_\x90V[_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x01\x8BWa\x05\xD9\x91a\x05\x8AV[PT`@\x1C\x90V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x05\x10Wa\x06\x03\x91`\x01\x82\x01\x81Ua\x05\x8AV[a\x06DW\x81Q` \x80\x84\x01Q`@\x94\x85\x01Q\x90\x94\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x93\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10a\x07\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a\x01\x8BW_\x19\x81\x01\x90\x81\x11a\x01\x8BWc\xFF\xFF\xFF\xFF\x16\x84_R\x82` Rc\xFF\xFF\xFF\xFFa\x074\x82`@_ a\x05\x8AV[PT\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a\x07MWP`\x01\x01a\x06hV[\x94PPPPP\x90V\xFE\xA2dipfsX\"\x12 \xDD\xBE\xF2\xF4M\xEDT\x14\xD4d\xA8v\x82\x1C{\x96\xC9I\xFBA\x15\x97\xCE\x0B\xE9\xED\xCA~>\xFA\r\xDCdsolcC\0\x08\x1B\x003",
    );
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`QuorumBitmapHistoryLib`](self) contract instance.

See the [wrapper's documentation](`QuorumBitmapHistoryLibInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> QuorumBitmapHistoryLibInstance<T, P, N> {
        QuorumBitmapHistoryLibInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<QuorumBitmapHistoryLibInstance<T, P, N>>,
    > {
        QuorumBitmapHistoryLibInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        QuorumBitmapHistoryLibInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`QuorumBitmapHistoryLib`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`QuorumBitmapHistoryLib`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct QuorumBitmapHistoryLibInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for QuorumBitmapHistoryLibInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("QuorumBitmapHistoryLibInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > QuorumBitmapHistoryLibInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`QuorumBitmapHistoryLib`](self) contract instance.

See the [wrapper's documentation](`QuorumBitmapHistoryLibInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<QuorumBitmapHistoryLibInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> QuorumBitmapHistoryLibInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> QuorumBitmapHistoryLibInstance<T, P, N> {
            QuorumBitmapHistoryLibInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > QuorumBitmapHistoryLibInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > QuorumBitmapHistoryLibInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
