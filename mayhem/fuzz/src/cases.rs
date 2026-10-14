// Two fieldless enums of 255 and 256 cases (case i is named "c<i>"), the derives, and the component that lifts them.
#![allow(dead_code)]
use wasmtime::component::{Component, ComponentType, Lift, Linker, Lower};
use wasmtime::{Config, Engine, Store};

#[derive(ComponentType, Lift, Lower, Clone, Copy)]
#[component(enum)]
#[repr(u8)]
pub enum E255 {
    #[component(name = "c0")]
    C0,
    #[component(name = "c1")]
    C1,
    #[component(name = "c2")]
    C2,
    #[component(name = "c3")]
    C3,
    #[component(name = "c4")]
    C4,
    #[component(name = "c5")]
    C5,
    #[component(name = "c6")]
    C6,
    #[component(name = "c7")]
    C7,
    #[component(name = "c8")]
    C8,
    #[component(name = "c9")]
    C9,
    #[component(name = "c10")]
    C10,
    #[component(name = "c11")]
    C11,
    #[component(name = "c12")]
    C12,
    #[component(name = "c13")]
    C13,
    #[component(name = "c14")]
    C14,
    #[component(name = "c15")]
    C15,
    #[component(name = "c16")]
    C16,
    #[component(name = "c17")]
    C17,
    #[component(name = "c18")]
    C18,
    #[component(name = "c19")]
    C19,
    #[component(name = "c20")]
    C20,
    #[component(name = "c21")]
    C21,
    #[component(name = "c22")]
    C22,
    #[component(name = "c23")]
    C23,
    #[component(name = "c24")]
    C24,
    #[component(name = "c25")]
    C25,
    #[component(name = "c26")]
    C26,
    #[component(name = "c27")]
    C27,
    #[component(name = "c28")]
    C28,
    #[component(name = "c29")]
    C29,
    #[component(name = "c30")]
    C30,
    #[component(name = "c31")]
    C31,
    #[component(name = "c32")]
    C32,
    #[component(name = "c33")]
    C33,
    #[component(name = "c34")]
    C34,
    #[component(name = "c35")]
    C35,
    #[component(name = "c36")]
    C36,
    #[component(name = "c37")]
    C37,
    #[component(name = "c38")]
    C38,
    #[component(name = "c39")]
    C39,
    #[component(name = "c40")]
    C40,
    #[component(name = "c41")]
    C41,
    #[component(name = "c42")]
    C42,
    #[component(name = "c43")]
    C43,
    #[component(name = "c44")]
    C44,
    #[component(name = "c45")]
    C45,
    #[component(name = "c46")]
    C46,
    #[component(name = "c47")]
    C47,
    #[component(name = "c48")]
    C48,
    #[component(name = "c49")]
    C49,
    #[component(name = "c50")]
    C50,
    #[component(name = "c51")]
    C51,
    #[component(name = "c52")]
    C52,
    #[component(name = "c53")]
    C53,
    #[component(name = "c54")]
    C54,
    #[component(name = "c55")]
    C55,
    #[component(name = "c56")]
    C56,
    #[component(name = "c57")]
    C57,
    #[component(name = "c58")]
    C58,
    #[component(name = "c59")]
    C59,
    #[component(name = "c60")]
    C60,
    #[component(name = "c61")]
    C61,
    #[component(name = "c62")]
    C62,
    #[component(name = "c63")]
    C63,
    #[component(name = "c64")]
    C64,
    #[component(name = "c65")]
    C65,
    #[component(name = "c66")]
    C66,
    #[component(name = "c67")]
    C67,
    #[component(name = "c68")]
    C68,
    #[component(name = "c69")]
    C69,
    #[component(name = "c70")]
    C70,
    #[component(name = "c71")]
    C71,
    #[component(name = "c72")]
    C72,
    #[component(name = "c73")]
    C73,
    #[component(name = "c74")]
    C74,
    #[component(name = "c75")]
    C75,
    #[component(name = "c76")]
    C76,
    #[component(name = "c77")]
    C77,
    #[component(name = "c78")]
    C78,
    #[component(name = "c79")]
    C79,
    #[component(name = "c80")]
    C80,
    #[component(name = "c81")]
    C81,
    #[component(name = "c82")]
    C82,
    #[component(name = "c83")]
    C83,
    #[component(name = "c84")]
    C84,
    #[component(name = "c85")]
    C85,
    #[component(name = "c86")]
    C86,
    #[component(name = "c87")]
    C87,
    #[component(name = "c88")]
    C88,
    #[component(name = "c89")]
    C89,
    #[component(name = "c90")]
    C90,
    #[component(name = "c91")]
    C91,
    #[component(name = "c92")]
    C92,
    #[component(name = "c93")]
    C93,
    #[component(name = "c94")]
    C94,
    #[component(name = "c95")]
    C95,
    #[component(name = "c96")]
    C96,
    #[component(name = "c97")]
    C97,
    #[component(name = "c98")]
    C98,
    #[component(name = "c99")]
    C99,
    #[component(name = "c100")]
    C100,
    #[component(name = "c101")]
    C101,
    #[component(name = "c102")]
    C102,
    #[component(name = "c103")]
    C103,
    #[component(name = "c104")]
    C104,
    #[component(name = "c105")]
    C105,
    #[component(name = "c106")]
    C106,
    #[component(name = "c107")]
    C107,
    #[component(name = "c108")]
    C108,
    #[component(name = "c109")]
    C109,
    #[component(name = "c110")]
    C110,
    #[component(name = "c111")]
    C111,
    #[component(name = "c112")]
    C112,
    #[component(name = "c113")]
    C113,
    #[component(name = "c114")]
    C114,
    #[component(name = "c115")]
    C115,
    #[component(name = "c116")]
    C116,
    #[component(name = "c117")]
    C117,
    #[component(name = "c118")]
    C118,
    #[component(name = "c119")]
    C119,
    #[component(name = "c120")]
    C120,
    #[component(name = "c121")]
    C121,
    #[component(name = "c122")]
    C122,
    #[component(name = "c123")]
    C123,
    #[component(name = "c124")]
    C124,
    #[component(name = "c125")]
    C125,
    #[component(name = "c126")]
    C126,
    #[component(name = "c127")]
    C127,
    #[component(name = "c128")]
    C128,
    #[component(name = "c129")]
    C129,
    #[component(name = "c130")]
    C130,
    #[component(name = "c131")]
    C131,
    #[component(name = "c132")]
    C132,
    #[component(name = "c133")]
    C133,
    #[component(name = "c134")]
    C134,
    #[component(name = "c135")]
    C135,
    #[component(name = "c136")]
    C136,
    #[component(name = "c137")]
    C137,
    #[component(name = "c138")]
    C138,
    #[component(name = "c139")]
    C139,
    #[component(name = "c140")]
    C140,
    #[component(name = "c141")]
    C141,
    #[component(name = "c142")]
    C142,
    #[component(name = "c143")]
    C143,
    #[component(name = "c144")]
    C144,
    #[component(name = "c145")]
    C145,
    #[component(name = "c146")]
    C146,
    #[component(name = "c147")]
    C147,
    #[component(name = "c148")]
    C148,
    #[component(name = "c149")]
    C149,
    #[component(name = "c150")]
    C150,
    #[component(name = "c151")]
    C151,
    #[component(name = "c152")]
    C152,
    #[component(name = "c153")]
    C153,
    #[component(name = "c154")]
    C154,
    #[component(name = "c155")]
    C155,
    #[component(name = "c156")]
    C156,
    #[component(name = "c157")]
    C157,
    #[component(name = "c158")]
    C158,
    #[component(name = "c159")]
    C159,
    #[component(name = "c160")]
    C160,
    #[component(name = "c161")]
    C161,
    #[component(name = "c162")]
    C162,
    #[component(name = "c163")]
    C163,
    #[component(name = "c164")]
    C164,
    #[component(name = "c165")]
    C165,
    #[component(name = "c166")]
    C166,
    #[component(name = "c167")]
    C167,
    #[component(name = "c168")]
    C168,
    #[component(name = "c169")]
    C169,
    #[component(name = "c170")]
    C170,
    #[component(name = "c171")]
    C171,
    #[component(name = "c172")]
    C172,
    #[component(name = "c173")]
    C173,
    #[component(name = "c174")]
    C174,
    #[component(name = "c175")]
    C175,
    #[component(name = "c176")]
    C176,
    #[component(name = "c177")]
    C177,
    #[component(name = "c178")]
    C178,
    #[component(name = "c179")]
    C179,
    #[component(name = "c180")]
    C180,
    #[component(name = "c181")]
    C181,
    #[component(name = "c182")]
    C182,
    #[component(name = "c183")]
    C183,
    #[component(name = "c184")]
    C184,
    #[component(name = "c185")]
    C185,
    #[component(name = "c186")]
    C186,
    #[component(name = "c187")]
    C187,
    #[component(name = "c188")]
    C188,
    #[component(name = "c189")]
    C189,
    #[component(name = "c190")]
    C190,
    #[component(name = "c191")]
    C191,
    #[component(name = "c192")]
    C192,
    #[component(name = "c193")]
    C193,
    #[component(name = "c194")]
    C194,
    #[component(name = "c195")]
    C195,
    #[component(name = "c196")]
    C196,
    #[component(name = "c197")]
    C197,
    #[component(name = "c198")]
    C198,
    #[component(name = "c199")]
    C199,
    #[component(name = "c200")]
    C200,
    #[component(name = "c201")]
    C201,
    #[component(name = "c202")]
    C202,
    #[component(name = "c203")]
    C203,
    #[component(name = "c204")]
    C204,
    #[component(name = "c205")]
    C205,
    #[component(name = "c206")]
    C206,
    #[component(name = "c207")]
    C207,
    #[component(name = "c208")]
    C208,
    #[component(name = "c209")]
    C209,
    #[component(name = "c210")]
    C210,
    #[component(name = "c211")]
    C211,
    #[component(name = "c212")]
    C212,
    #[component(name = "c213")]
    C213,
    #[component(name = "c214")]
    C214,
    #[component(name = "c215")]
    C215,
    #[component(name = "c216")]
    C216,
    #[component(name = "c217")]
    C217,
    #[component(name = "c218")]
    C218,
    #[component(name = "c219")]
    C219,
    #[component(name = "c220")]
    C220,
    #[component(name = "c221")]
    C221,
    #[component(name = "c222")]
    C222,
    #[component(name = "c223")]
    C223,
    #[component(name = "c224")]
    C224,
    #[component(name = "c225")]
    C225,
    #[component(name = "c226")]
    C226,
    #[component(name = "c227")]
    C227,
    #[component(name = "c228")]
    C228,
    #[component(name = "c229")]
    C229,
    #[component(name = "c230")]
    C230,
    #[component(name = "c231")]
    C231,
    #[component(name = "c232")]
    C232,
    #[component(name = "c233")]
    C233,
    #[component(name = "c234")]
    C234,
    #[component(name = "c235")]
    C235,
    #[component(name = "c236")]
    C236,
    #[component(name = "c237")]
    C237,
    #[component(name = "c238")]
    C238,
    #[component(name = "c239")]
    C239,
    #[component(name = "c240")]
    C240,
    #[component(name = "c241")]
    C241,
    #[component(name = "c242")]
    C242,
    #[component(name = "c243")]
    C243,
    #[component(name = "c244")]
    C244,
    #[component(name = "c245")]
    C245,
    #[component(name = "c246")]
    C246,
    #[component(name = "c247")]
    C247,
    #[component(name = "c248")]
    C248,
    #[component(name = "c249")]
    C249,
    #[component(name = "c250")]
    C250,
    #[component(name = "c251")]
    C251,
    #[component(name = "c252")]
    C252,
    #[component(name = "c253")]
    C253,
    #[component(name = "c254")]
    C254,
}

#[derive(ComponentType, Lift, Lower, Clone, Copy)]
#[component(enum)]
#[repr(u8)]
pub enum E256 {
    #[component(name = "c0")]
    C0,
    #[component(name = "c1")]
    C1,
    #[component(name = "c2")]
    C2,
    #[component(name = "c3")]
    C3,
    #[component(name = "c4")]
    C4,
    #[component(name = "c5")]
    C5,
    #[component(name = "c6")]
    C6,
    #[component(name = "c7")]
    C7,
    #[component(name = "c8")]
    C8,
    #[component(name = "c9")]
    C9,
    #[component(name = "c10")]
    C10,
    #[component(name = "c11")]
    C11,
    #[component(name = "c12")]
    C12,
    #[component(name = "c13")]
    C13,
    #[component(name = "c14")]
    C14,
    #[component(name = "c15")]
    C15,
    #[component(name = "c16")]
    C16,
    #[component(name = "c17")]
    C17,
    #[component(name = "c18")]
    C18,
    #[component(name = "c19")]
    C19,
    #[component(name = "c20")]
    C20,
    #[component(name = "c21")]
    C21,
    #[component(name = "c22")]
    C22,
    #[component(name = "c23")]
    C23,
    #[component(name = "c24")]
    C24,
    #[component(name = "c25")]
    C25,
    #[component(name = "c26")]
    C26,
    #[component(name = "c27")]
    C27,
    #[component(name = "c28")]
    C28,
    #[component(name = "c29")]
    C29,
    #[component(name = "c30")]
    C30,
    #[component(name = "c31")]
    C31,
    #[component(name = "c32")]
    C32,
    #[component(name = "c33")]
    C33,
    #[component(name = "c34")]
    C34,
    #[component(name = "c35")]
    C35,
    #[component(name = "c36")]
    C36,
    #[component(name = "c37")]
    C37,
    #[component(name = "c38")]
    C38,
    #[component(name = "c39")]
    C39,
    #[component(name = "c40")]
    C40,
    #[component(name = "c41")]
    C41,
    #[component(name = "c42")]
    C42,
    #[component(name = "c43")]
    C43,
    #[component(name = "c44")]
    C44,
    #[component(name = "c45")]
    C45,
    #[component(name = "c46")]
    C46,
    #[component(name = "c47")]
    C47,
    #[component(name = "c48")]
    C48,
    #[component(name = "c49")]
    C49,
    #[component(name = "c50")]
    C50,
    #[component(name = "c51")]
    C51,
    #[component(name = "c52")]
    C52,
    #[component(name = "c53")]
    C53,
    #[component(name = "c54")]
    C54,
    #[component(name = "c55")]
    C55,
    #[component(name = "c56")]
    C56,
    #[component(name = "c57")]
    C57,
    #[component(name = "c58")]
    C58,
    #[component(name = "c59")]
    C59,
    #[component(name = "c60")]
    C60,
    #[component(name = "c61")]
    C61,
    #[component(name = "c62")]
    C62,
    #[component(name = "c63")]
    C63,
    #[component(name = "c64")]
    C64,
    #[component(name = "c65")]
    C65,
    #[component(name = "c66")]
    C66,
    #[component(name = "c67")]
    C67,
    #[component(name = "c68")]
    C68,
    #[component(name = "c69")]
    C69,
    #[component(name = "c70")]
    C70,
    #[component(name = "c71")]
    C71,
    #[component(name = "c72")]
    C72,
    #[component(name = "c73")]
    C73,
    #[component(name = "c74")]
    C74,
    #[component(name = "c75")]
    C75,
    #[component(name = "c76")]
    C76,
    #[component(name = "c77")]
    C77,
    #[component(name = "c78")]
    C78,
    #[component(name = "c79")]
    C79,
    #[component(name = "c80")]
    C80,
    #[component(name = "c81")]
    C81,
    #[component(name = "c82")]
    C82,
    #[component(name = "c83")]
    C83,
    #[component(name = "c84")]
    C84,
    #[component(name = "c85")]
    C85,
    #[component(name = "c86")]
    C86,
    #[component(name = "c87")]
    C87,
    #[component(name = "c88")]
    C88,
    #[component(name = "c89")]
    C89,
    #[component(name = "c90")]
    C90,
    #[component(name = "c91")]
    C91,
    #[component(name = "c92")]
    C92,
    #[component(name = "c93")]
    C93,
    #[component(name = "c94")]
    C94,
    #[component(name = "c95")]
    C95,
    #[component(name = "c96")]
    C96,
    #[component(name = "c97")]
    C97,
    #[component(name = "c98")]
    C98,
    #[component(name = "c99")]
    C99,
    #[component(name = "c100")]
    C100,
    #[component(name = "c101")]
    C101,
    #[component(name = "c102")]
    C102,
    #[component(name = "c103")]
    C103,
    #[component(name = "c104")]
    C104,
    #[component(name = "c105")]
    C105,
    #[component(name = "c106")]
    C106,
    #[component(name = "c107")]
    C107,
    #[component(name = "c108")]
    C108,
    #[component(name = "c109")]
    C109,
    #[component(name = "c110")]
    C110,
    #[component(name = "c111")]
    C111,
    #[component(name = "c112")]
    C112,
    #[component(name = "c113")]
    C113,
    #[component(name = "c114")]
    C114,
    #[component(name = "c115")]
    C115,
    #[component(name = "c116")]
    C116,
    #[component(name = "c117")]
    C117,
    #[component(name = "c118")]
    C118,
    #[component(name = "c119")]
    C119,
    #[component(name = "c120")]
    C120,
    #[component(name = "c121")]
    C121,
    #[component(name = "c122")]
    C122,
    #[component(name = "c123")]
    C123,
    #[component(name = "c124")]
    C124,
    #[component(name = "c125")]
    C125,
    #[component(name = "c126")]
    C126,
    #[component(name = "c127")]
    C127,
    #[component(name = "c128")]
    C128,
    #[component(name = "c129")]
    C129,
    #[component(name = "c130")]
    C130,
    #[component(name = "c131")]
    C131,
    #[component(name = "c132")]
    C132,
    #[component(name = "c133")]
    C133,
    #[component(name = "c134")]
    C134,
    #[component(name = "c135")]
    C135,
    #[component(name = "c136")]
    C136,
    #[component(name = "c137")]
    C137,
    #[component(name = "c138")]
    C138,
    #[component(name = "c139")]
    C139,
    #[component(name = "c140")]
    C140,
    #[component(name = "c141")]
    C141,
    #[component(name = "c142")]
    C142,
    #[component(name = "c143")]
    C143,
    #[component(name = "c144")]
    C144,
    #[component(name = "c145")]
    C145,
    #[component(name = "c146")]
    C146,
    #[component(name = "c147")]
    C147,
    #[component(name = "c148")]
    C148,
    #[component(name = "c149")]
    C149,
    #[component(name = "c150")]
    C150,
    #[component(name = "c151")]
    C151,
    #[component(name = "c152")]
    C152,
    #[component(name = "c153")]
    C153,
    #[component(name = "c154")]
    C154,
    #[component(name = "c155")]
    C155,
    #[component(name = "c156")]
    C156,
    #[component(name = "c157")]
    C157,
    #[component(name = "c158")]
    C158,
    #[component(name = "c159")]
    C159,
    #[component(name = "c160")]
    C160,
    #[component(name = "c161")]
    C161,
    #[component(name = "c162")]
    C162,
    #[component(name = "c163")]
    C163,
    #[component(name = "c164")]
    C164,
    #[component(name = "c165")]
    C165,
    #[component(name = "c166")]
    C166,
    #[component(name = "c167")]
    C167,
    #[component(name = "c168")]
    C168,
    #[component(name = "c169")]
    C169,
    #[component(name = "c170")]
    C170,
    #[component(name = "c171")]
    C171,
    #[component(name = "c172")]
    C172,
    #[component(name = "c173")]
    C173,
    #[component(name = "c174")]
    C174,
    #[component(name = "c175")]
    C175,
    #[component(name = "c176")]
    C176,
    #[component(name = "c177")]
    C177,
    #[component(name = "c178")]
    C178,
    #[component(name = "c179")]
    C179,
    #[component(name = "c180")]
    C180,
    #[component(name = "c181")]
    C181,
    #[component(name = "c182")]
    C182,
    #[component(name = "c183")]
    C183,
    #[component(name = "c184")]
    C184,
    #[component(name = "c185")]
    C185,
    #[component(name = "c186")]
    C186,
    #[component(name = "c187")]
    C187,
    #[component(name = "c188")]
    C188,
    #[component(name = "c189")]
    C189,
    #[component(name = "c190")]
    C190,
    #[component(name = "c191")]
    C191,
    #[component(name = "c192")]
    C192,
    #[component(name = "c193")]
    C193,
    #[component(name = "c194")]
    C194,
    #[component(name = "c195")]
    C195,
    #[component(name = "c196")]
    C196,
    #[component(name = "c197")]
    C197,
    #[component(name = "c198")]
    C198,
    #[component(name = "c199")]
    C199,
    #[component(name = "c200")]
    C200,
    #[component(name = "c201")]
    C201,
    #[component(name = "c202")]
    C202,
    #[component(name = "c203")]
    C203,
    #[component(name = "c204")]
    C204,
    #[component(name = "c205")]
    C205,
    #[component(name = "c206")]
    C206,
    #[component(name = "c207")]
    C207,
    #[component(name = "c208")]
    C208,
    #[component(name = "c209")]
    C209,
    #[component(name = "c210")]
    C210,
    #[component(name = "c211")]
    C211,
    #[component(name = "c212")]
    C212,
    #[component(name = "c213")]
    C213,
    #[component(name = "c214")]
    C214,
    #[component(name = "c215")]
    C215,
    #[component(name = "c216")]
    C216,
    #[component(name = "c217")]
    C217,
    #[component(name = "c218")]
    C218,
    #[component(name = "c219")]
    C219,
    #[component(name = "c220")]
    C220,
    #[component(name = "c221")]
    C221,
    #[component(name = "c222")]
    C222,
    #[component(name = "c223")]
    C223,
    #[component(name = "c224")]
    C224,
    #[component(name = "c225")]
    C225,
    #[component(name = "c226")]
    C226,
    #[component(name = "c227")]
    C227,
    #[component(name = "c228")]
    C228,
    #[component(name = "c229")]
    C229,
    #[component(name = "c230")]
    C230,
    #[component(name = "c231")]
    C231,
    #[component(name = "c232")]
    C232,
    #[component(name = "c233")]
    C233,
    #[component(name = "c234")]
    C234,
    #[component(name = "c235")]
    C235,
    #[component(name = "c236")]
    C236,
    #[component(name = "c237")]
    C237,
    #[component(name = "c238")]
    C238,
    #[component(name = "c239")]
    C239,
    #[component(name = "c240")]
    C240,
    #[component(name = "c241")]
    C241,
    #[component(name = "c242")]
    C242,
    #[component(name = "c243")]
    C243,
    #[component(name = "c244")]
    C244,
    #[component(name = "c245")]
    C245,
    #[component(name = "c246")]
    C246,
    #[component(name = "c247")]
    C247,
    #[component(name = "c248")]
    C248,
    #[component(name = "c249")]
    C249,
    #[component(name = "c250")]
    C250,
    #[component(name = "c251")]
    C251,
    #[component(name = "c252")]
    C252,
    #[component(name = "c253")]
    C253,
    #[component(name = "c254")]
    C254,
    #[component(name = "c255")]
    C255,
}

pub fn engine() -> Engine {
    let mut config = Config::new();
    config.memory_reservation(1 << 20);
    config.memory_guard_size(0);
    Engine::new(&config).unwrap()
}

// A component whose `f(a: u8, b: u8)` returns `tuple<e, e>` holding (a, b): two flat results do not fit
// in a single return value, so the host lifts both enums back out of linear memory.
pub fn component(engine: &Engine, cases: usize) -> Component {
    let names: String = (0..cases).map(|i| format!(" \"c{i}\"")).collect();
    let wat = format!(
        r#"(component
  (type $e (enum{names}))
  (export $E "e" (type $e))
  (core module $m
    (memory (export "memory") 1)
    (func (export "f") (param i32 i32) (result i32)
      (i32.store8 (i32.const 0) (local.get 0))
      (i32.store8 (i32.const 1) (local.get 1))
      (i32.const 0)))
  (core instance $i (instantiate $m))
  (alias core export $i "f" (core func $f))
  (func (export "f") (param "a" u8) (param "b" u8) (result (tuple $E $E))
    (canon lift (core func $f) (memory (core memory $i "memory")))))"#
    );
    Component::new(engine, wat).unwrap()
}

macro_rules! lifter {
    ($name:ident, $e:ty) => {
        pub fn $name(engine: &Engine, component: &Component, a: u8, b: u8) -> wasmtime::Result<($e, $e)> {
            let mut store = Store::new(engine, ());
            let instance = Linker::new(engine).instantiate(&mut store, component)?;
            let f = instance.get_typed_func::<(u8, u8), (($e, $e),)>(&mut store, "f")?;
            let (pair,) = f.call(&mut store, (a, b))?;
            f.post_return(&mut store)?;
            Ok(pair)
        }
    };
}

lifter!(lift_e255, E255);
lifter!(lift_e256, E256);
