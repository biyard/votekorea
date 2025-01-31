use dioxus_translate::*;

translate! {
    PagesTranslate;

    _text: {
        ko: "홈페이지",
        en: "homepage",
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
        ko: "투자 홍보 후원금은 해당 청원을 많은 사람에게 알리기 위해 사용됩니다.",
        en: "The promotion investment fund is used to promote the petition to many people.",
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
