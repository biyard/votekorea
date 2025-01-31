use dioxus_translate::*;

translate! {
    PagesTranslate;

    _text: {
        ko: "홈페이지",
        en: "homepage",
    },

    blockchain_vote_title1: {
        ko: "블록체인 국민투표란?",
        en: "What is blockchain voting?",
    },

    blockchain_vote_title2: {
        ko: "더욱 신뢰있는 대한민국 여론을 만듭니다.",
        en: "Create more trustworthy public opinion in Korea.",
    },

    blockchain_vote_tech_title: {
        ko: "기술로 조작을 방지합니다.",
        en: "Prevent manipulation with technology.",
    },

    blockchain_vote_tech_content: {
        ko: "블록체인에 1인 1투표 원칙으로 등록된 기록은 변경되지 않습니다.",
        en: "Records registered with the principle of one person, one vote on the blockchain cannot be changed.",
    },

    blockchain_vote_org_title: {
        ko: "전문기관이 조작을 방지합니다.",
        en: "Professional organizations prevent manipulation.",
    },

    blockchain_vote_org_content: {
        ko: "국제기구 및 블록체인 전문기관이 기술과 플랫폼의 무결성을 확인합니다.",
        en: "International organizations and blockchain experts verify the integrity of technology and platforms.",
    },

    blockchain_vote_transparency_title: {
        ko: "다수의 참여로 조작을 방지합니다.",
        en: "Prevent manipulation with the participation of many people.",
    },

    blockchain_vote_transparency_content: {
        ko: "기존 여론조사 참여 규모의 한계를 극복하여 더욱 정확한 데이터를 제공합니다.",
        en: "Overcome the limitations of the scale of participation in existing opinion polls to provide more accurate data.",
    },
}

translate! {
    AboutTranslate;

    title: {
        ko: "국민투표가 추구하는 가치",
        en: "Values pursued by national voting",
    },

    content1: {
        ko: "선거과정에 대한 신뢰는 민주주의를 지탱하는 근간입니다. 그러나 최근 데이터 조작과 왜곡에 대한 의혹이 여론조사와 선거의 결과에 대한 신뢰를 무너트려 혼란이 증폭되고 있습니다. ",
        en: "Trust in the electoral process is the foundation of democracy. However, suspicions of data manipulation and distortion have been undermining trust in the results of opinion polls and elections, leading to increased confusion.",
    },

    content2: {
        ko: "이를 해결하기 위해 본 프로젝트는 블록체인 기술을 활용하여 데이터 변조를 원천적으로 차단하고, 투표 과정을 투명하게 공개합니다. 그 위에 국제 검증 기구를 통한 감사로 시스템의 중립성과 신뢰성을 복구, 강화했습니다.",
        en: "To address this, this project uses blockchain technology to prevent data tampering at the source and transparently discloses the voting process. On top of that, the neutrality and reliability of the system were restored and strengthened through audits by international verification organizations.",
    },

    trans_title: {
        ko: "투명성",
        en: "Transparency",
    },
    trans_content: {
        ko: "데이터는 블록체인에 기록되어 누구나 실시간으로 검증할 수 있습니다.",
        en: "Data is recorded on the blockchain and can be verified in real time by anyone.",
    },

    fair_title: {
        ko: "공정성",
        en: "Fairness",
    },
    fair_content: {
        ko: "운영자를 포함한 특정 이익집단의 조작이 불가능하도록 설계합니다.",
        en: "Designed to prevent manipulation by specific interest groups, including operators.",
    },

    secure_title: {
        ko: "안전성",
        en: "Safety",
    },
    secure_content: {
        ko: "블록체인을 통해 데이터를 보호하며 외부 개입을 차단합니다.",
        en: "Protects data through blockchain and blocks external interference.",
    },

    public_title: {
        ko: "공공성",
        en: "Publicness",
    },
    public_content: {
        ko: "정부나 기관이 필요에 따라 사용할 수 있도록 오픈소스로 운영됩니다.",
        en: "Operated as open source so that governments and institutions can use it as needed.",
    },
}

translate! {
    HighlightedTopicTranslate;

    unit: {
        ko: "명",
        en: "people",
    },

    currency: {
        ko: "원",
        en: "won",
    },

    period_tooltip: {
        ko: "투표 기간",
        en: "Voting period",
    },

    requirement_tooltip: {
        ko: "청원 통과 요구 인원",
        en: "Number of people required to pass the petition",
    },

    amount_title: {
        ko: "투표 홍보 후원금",
        en: "Promotion investment fund",
    }
    amount_tooltip: {
        ko: "본 사업은 공공 목적으로 비영리 운영이 원칙이며, 모인 후원금은 전액 투표의 운영 및 결과의 대국민적 홍보를 위해 사용됩니다.",
        en: "This project is operated on a non-profit basis for public purposes, and the collected donations will be used for the operation of the entire vote and the national promotion of the results.",
    },
    remaining_tooltip: {
        ko: "청원 통과까지 남은 투표 인원",
        en: "Number of votes remaining until the petition is passed",
    },

    btn_support: {
        ko: "지지하기",
        en: "Support",
    },
    voted: {
        ko: "이미 지지한 청원입니다.",
        en: "You have already supported this petition.",
    },
}

translate! {
    ContentWrapperTranslate;

    cumulative_donations: {
        ko: "누적 기부금",
        en: "Cumulative Donations",
    },
}

translate! {
    HeaderTranslate;

    signup: {
        ko: "회원가입",
        en: "Sign up",
    },

    login: {
        ko: "로그인",
        en: "Login",
    },

    logout: {
        ko: "로그아웃",
        en: "Logout",
    },
}

translate! {
    CongratulationPopupTranslate;

    welcome: {
        ko: "환영합니다!",
        en: "Welcome!",
    },

    congratulation: {
        ko: "‘서비스명’에 오신 것을 환영합니다!\n익명성과 신뢰를 바탕으로 안전한 투표 환경을 제공합니다.",
        en: "Welcome to 'Service Name'!\nWe provide a safe voting environment based on anonymity and trust.",
    },

    start_poll: {
        ko: "투표 시작하기",
        en: "Start voting",
    },
}

translate! {
    UserSetupPopupTranslate;

    welcome: {
        ko: "환영합니다!",
        en: "Welcome!",
    },

    enter_nickname: {
        ko: "닉네임을 입력해주세요",
        en: "Please enter your nickname",
    },

    special_characters: {
        ko: "특수문자는 입력할 수 없습니다.",
        en: "Special characters are not allowed.",
    },

    agree_email: {
        ko: "[필수]이메일 및 계정주소 수집에 동의합니다.",
        en: "[Required]I agree to collect email and account address.",
    },

    next: {
        ko: "다음",
        en: "Next",
    },
}

translate! {
    SignupPopupTranslate;

    continue_with_kakao: {
        ko: "카카오 로그인",
        en: "Continue with Kakao",
    },
}

translate! {
    MenusTranslate;

    home: {
        ko: "HOME",
        en: "HOME",
    },
    blockchain_vote: {
        ko: "블록체인 국민투표란?",
        en: "What is blockchain voting?",
    },
    about: {
        ko: "ABOUT",
        en: "ABOUT",
    },
    contact_us: {
        ko: "CONTACT US",
        en: "CONTACT US",
    },
    topic: {
        ko: "TOPIC",
        en: "TOPIC",
    },
}

translate! {
    VotingPopupTranslate;

    title: {
        ko: "청원 지지하기",
        en: "Support the petition",
    },

    agreement: {
        ko: "(선택) 해당 청원을 홍보하기 위한 후원금을 지원하겠습니까?",
        en: "(Optional) Would you like to support the petition with a donation to promote it?",
    },

    label_title: {
        ko: "청원주제",
        en: "Petition title",
    },
    label_amount: {
        ko: "후원금액",
        en: "Donation",
    },

    label_name: {
        ko: "입금자명",
        en: "Donor name",
    },

    placeholder_name: {
        ko: "이체시 받으실 분에 보여지는 이름을 입력해주세요.",
        en: "Please enter the name that will be shown to the recipient when transferring.",
    },

    notice: {
        ko: "홍보후원금은 아래의 계좌로 입금해주세요.",
        en: "Please transfer the promotion donation to the account below.",
    },

    account_info: {
        ko: "계좌번호: OO은행 / XXX-XXX-XXXXXX",
        en: "Account number: OO Bank / XXX-XXX-XXXXXX",
    },

    account_owner: {
        ko: "예금주: OO",
        en: "Account owner: OO",
    },

    update: {
        ko: "홍보후원금액은 OO시에 업데이트 됩니다.",
        en: "The promotion donation amount will be updated at OO.",
    },

    support: {
        ko: "지지하기",
        en: "Support",
    },

}

translate! {
    VoteResultHorizontalBarsTranslate;

    people_unit: {
        ko: "명",
        en: "people",
    },
}

translate! {
    FooterTranslate;

    service_name: {
        ko: "국민투표",
        en: "National Voting",
    },

    service_desc: {
        ko: "Mine the Future, Cast Your Predictions.",
        en: "Mine the Future, Cast Your Predictions.",
    },

    info1: {
        ko: "비영리임의단체 시민의목소리 (VoxPopuli) | 000-00-000 | 대표 정상규",
        en: "Non-profit organization Citizen's Voice (VoxPopuli) | 000-00-000 | Representative Sanggyu Jeong",
    },

    info2: {
        ko: "(06236) 서울시 강남구 테헤란로 20길 19 | Email info@voxpopuli.net",
        en: "19, Teheran-ro 20-gil, Gangnam-gu, Seoul 06236 | Email",
    },

    copyright: {
        ko: "© 2025  VoxPopuli. All rights reserved.",
        en: "© 2025  VoxPopuli. All rights reserved.",
    },

    privacy_policy: {
        ko: "개인정보처리방침",
        en: "Privacy Policy",
    },

    terms_of_service: {
        ko: "이용약관",
        en: "Terms of Service",
    },
}
