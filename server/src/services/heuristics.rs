use std::collections::HashSet;

#[derive(Debug)]
pub struct HeuristicResult {
    pub score: u8,
    pub signals: Vec<String>,
}

const FORMULAIC_PHRASES: &[&str] = &[
    // Classic AI filler
    "in today's world",
    "it's important to note",
    "it is important to note",
    "it's worth noting",
    "it is worth noting",
    "in conclusion",
    "to sum up",
    "all things considered",
    "at the end of the day",
    "in this article",
    "here's the thing",
    "without further ado",
    "that being said",
    "having said that",
    "let's dive in",
    "dive into",
    "delve into",
    "let's explore",
    "in the world of",
    "in the realm of",
    "in light of",
    "to some extent",
    "in many cases",
    "it can be argued",
    "studies have shown",
    "experts agree",
    // Buzzwords
    "game changer",
    "game-changer",
    "cutting-edge",
    "paradigm shift",
    "holistic approach",
    "thought leader",
    "value proposition",
    "best practices",
    "circle back",
    "unpack this",
    "at its core",
    "it goes without saying",
    "comprehensive guide",
    "treasure trove",
    "tapestry of",
    "daunting task",
    // AI vocabulary
    "leverage",
    "revolutionize",
    "seamlessly",
    "furthermore",
    "moreover",
    "additionally",
    "subsequently",
    "navigate the complexities",
    "supercharge",
    "unleash",
    "unlock",
    "harness",
    "robust",
    "transformative",
    "synergy",
    "confluence",
    "pivotal",
    "myriad",
    "plethora",
    "arguably",
];

/// AI-associated standalone words — checked as whole words, case-insensitive.
const AI_VOCABULARY: &[&str] = &[
    "underpinning",
    "trajectory",
    "spectrum",
    "facet",
    "intricacies",
    "iterative",
    "nuanced",
    "holistic",
    "dynamic",
    "framework",
    "comprehensive",
    "innovative",
    "bustling",
    "remarkable",
    "excitingly",
    "turbocharging",
    "unveiling",
    "harnessing",
    "revolutionizing",
    "unleashing",
    "unlocking",
];

/// Slang / abbreviations that humans use — checked as whole words.
const HUMAN_SLANG: &[&str] = &[
    "lol", "lmao", "rofl", "tbh", "fr", "smh", "ngl", "bruh", "omg", "wtf", "idk", "imo",
    "imho", "fwiw", "afaik", "btw", "irl", "fomo", "goat", "nah", "yep", "yup", "haha",
    "hehe", "oops", "ugh", "meh", "pls", "plz", "thx", "ty",
];

/// Casual contractions that signal human writing.
const CASUAL_CONTRACTIONS: &[&str] = &[
    "gonna", "wanna", "kinda", "gotta", "dunno", "ain't", "y'all", "can't even",
    "lowkey", "highkey", "deadass", "legit",
];

/// Promotional / motivational patterns common in AI-generated social media.
const PROMOTIONAL_PATTERNS: &[&str] = &[
    // CTAs
    "link in bio",
    "link in comments",
    "link in the comments",
    "dm me",
    "follow for more",
    "comment below",
    "share this",
    "tag someone",
    "save this post",
    "bookmark this",
    "check it out",
    "don't miss out",
    "sign up",
    "star if you",
    "please star",
    "repost if",
    "repost this",
    // Motivational / hustle culture
    "top 1%",
    "99% won't",
    "99% of people",
    "most people don't",
    "most people won't",
    "successful people",
    "the secret is",
    "here's what i",
    "here's how i",
    "here are the",
    "stop doing",
    "start doing",
    "the truth is",
    "nobody tells you",
    "no one tells you",
    "changed my life",
    "you need to know",
    "the hard truth",
    "key takeaway",
    "quick thread",
    "unpopular opinion",
    "hot take",
    // Listicle / thread openers
    "here are",
    "things i learned",
    "lessons i learned",
    "mistakes i made",
];

pub fn analyze(text: &str) -> HeuristicResult {
    let mut signals = Vec::new();
    let mut score_sum: f64 = 0.0;
    let mut weight_sum: f64 = 0.0;

    // 1. Sentence length variance (AI tends to write uniform sentence lengths)
    //    Neutral baseline: 4.0 (no evidence either way)
    let sentence_variance = sentence_length_variance(text);
    let sv_score = if sentence_variance < 5.0 {
        signals.push("uniform_sentence_length".to_string());
        8.0
    } else if sentence_variance < 15.0 {
        signals.push("low_sentence_variance".to_string());
        6.0
    } else if sentence_variance > 50.0 {
        2.0 // very varied = human
    } else {
        4.0
    };
    score_sum += sv_score * 2.0;
    weight_sum += 2.0;

    // 2. Vocabulary diversity (Type-Token Ratio)
    //    High diversity is slightly human-leaning
    let ttr = type_token_ratio(text);
    let ttr_score = if ttr < 0.4 {
        signals.push("low_vocabulary_diversity".to_string());
        7.0
    } else if ttr < 0.55 {
        5.0
    } else {
        3.0 // diverse vocab leans human
    };
    score_sum += ttr_score * 1.5;
    weight_sum += 1.5;

    // 3. Burstiness (AI text tends to have low burstiness — uniform flow)
    //    High burstiness is a genuine human signal, keep at 2.0
    let burstiness = compute_burstiness(text);
    let burst_score = if burstiness < 0.3 {
        signals.push("low_burstiness".to_string());
        7.0
    } else if burstiness < 0.5 {
        5.0
    } else {
        2.0 // bursty = human
    };
    score_sum += burst_score * 1.5;
    weight_sum += 1.5;

    // 4. Formulaic phrase detection
    //    No phrases = neutral (4.0), not "definitely human"
    let formula_count = count_formulaic_phrases(text);
    let formula_score = if formula_count >= 3 {
        signals.push("formulaic_phrases".to_string());
        9.0
    } else if formula_count >= 1 {
        signals.push("some_formulaic_phrases".to_string());
        6.0
    } else {
        4.0
    };
    score_sum += formula_score * 2.5;
    weight_sum += 2.5;

    // 5. Dash detection (em dash, en dash — strong AI indicator)
    let dash_count = count_dashes(text);
    let dash_score = if dash_count >= 3 {
        signals.push("excessive_dashes".to_string());
        9.0
    } else if dash_count >= 1 {
        signals.push("dash_usage".to_string());
        6.0
    } else {
        4.0
    };
    score_sum += dash_score * 2.0;
    weight_sum += 2.0;

    // 6. AI vocabulary words (standalone words, not just phrases)
    let ai_word_count = count_ai_vocabulary(text);
    let vocab_score = if ai_word_count >= 3 {
        signals.push("ai_vocabulary".to_string());
        8.0
    } else if ai_word_count >= 1 {
        signals.push("some_ai_vocabulary".to_string());
        6.0
    } else {
        4.0
    };
    score_sum += vocab_score * 1.5;
    weight_sum += 1.5;

    // 7. Punctuation patterns (AI uses more consistent punctuation)
    let punct_score = punctuation_analysis(text, &mut signals);
    score_sum += punct_score * 1.0;
    weight_sum += 1.0;

    // 8. Human informality markers (slang, casual language, !! / ??)
    //    Strong human signal when present; absence leans AI
    let informality = count_informality(text);
    let informal_score = if informality >= 3 {
        signals.push("informal_language".to_string());
        1.0
    } else if informality >= 1 {
        signals.push("some_informal_markers".to_string());
        3.0
    } else {
        5.0 // no casual markers at all leans AI
    };
    score_sum += informal_score * 2.0;
    weight_sum += 2.0;

    // 9. Line-break heavy formatting (LinkedIn AI: one sentence per line)
    let lb_ratio = linebreak_ratio(text);
    let lb_score = if lb_ratio > 0.8 {
        signals.push("line_per_sentence".to_string());
        7.0
    } else if lb_ratio > 0.5 {
        signals.push("heavy_line_breaks".to_string());
        6.0
    } else {
        4.0
    };
    score_sum += lb_score * 1.5;
    weight_sum += 1.5;

    // 10. Promotional / motivational patterns (social media AI)
    let promo_count = count_promotional(text);
    let promo_score = if promo_count >= 2 {
        signals.push("promotional_pattern".to_string());
        8.0
    } else if promo_count >= 1 {
        signals.push("some_promotional".to_string());
        6.0
    } else {
        4.0
    };
    score_sum += promo_score * 1.5;
    weight_sum += 1.5;

    // 11. Text too short for reliable analysis
    let word_count = text.split_whitespace().count();
    if word_count < 20 {
        signals.push("short_text_low_confidence".to_string());
    }

    let final_score = if weight_sum > 0.0 {
        (score_sum / weight_sum).round() as u8
    } else {
        5
    };

    HeuristicResult {
        score: final_score.min(10),
        signals,
    }
}

fn sentence_length_variance(text: &str) -> f64 {
    let sentences: Vec<&str> = text
        .split(|c: char| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.len() < 2 {
        return 50.0; // Not enough sentences to judge
    }

    let lengths: Vec<f64> = sentences
        .iter()
        .map(|s| s.split_whitespace().count() as f64)
        .collect();

    let mean = lengths.iter().sum::<f64>() / lengths.len() as f64;
    lengths.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / lengths.len() as f64
}

fn type_token_ratio(text: &str) -> f64 {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect();

    if words.is_empty() {
        return 1.0;
    }

    let unique: HashSet<&String> = words.iter().collect();
    unique.len() as f64 / words.len() as f64
}

fn compute_burstiness(text: &str) -> f64 {
    let sentences: Vec<&str> = text
        .split(|c: char| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.len() < 3 {
        return 0.5;
    }

    let lengths: Vec<f64> = sentences
        .iter()
        .map(|s| s.split_whitespace().count() as f64)
        .collect();

    let mean = lengths.iter().sum::<f64>() / lengths.len() as f64;
    if mean == 0.0 {
        return 0.5;
    }

    let std_dev = (lengths.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / lengths.len() as f64).sqrt();
    // Burstiness = (std - mean) / (std + mean), normalized to 0-1
    let raw = (std_dev - mean) / (std_dev + mean);
    (raw + 1.0) / 2.0 // Normalize from [-1,1] to [0,1]
}

fn count_formulaic_phrases(text: &str) -> usize {
    let lower = text.to_lowercase();
    FORMULAIC_PHRASES
        .iter()
        .filter(|phrase| lower.contains(**phrase))
        .count()
}

fn count_dashes(text: &str) -> usize {
    let mut count = 0;
    for ch in text.chars() {
        // Em dash (—), en dash (–)
        if ch == '\u{2014}' || ch == '\u{2013}' {
            count += 1;
        }
    }
    // Also detect spaced hyphens like " - " or " -- " (surrogate em dashes)
    count += text.matches(" - ").count();
    count += text.matches(" -- ").count();
    count
}

fn count_ai_vocabulary(text: &str) -> usize {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect();
    AI_VOCABULARY
        .iter()
        .filter(|vocab| words.iter().any(|w| *w == **vocab))
        .count()
}

fn punctuation_analysis(text: &str, signals: &mut Vec<String>) -> f64 {
    let sentences: Vec<&str> = text
        .split(|c: char| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.len() < 3 {
        return 4.0;
    }

    // Check if almost all sentences end with periods (low variety)
    let total_terminators = text.chars().filter(|c| *c == '.' || *c == '!' || *c == '?').count();
    if total_terminators == 0 {
        return 4.0;
    }

    let period_ratio = text.chars().filter(|c| *c == '.').count() as f64 / total_terminators as f64;
    if period_ratio > 0.95 {
        signals.push("uniform_punctuation".to_string());
        return 6.0;
    }

    // Check comma frequency (AI tends to use more commas)
    let comma_count = text.chars().filter(|c| *c == ',').count();
    let word_count = text.split_whitespace().count();
    if word_count > 0 {
        let comma_ratio = comma_count as f64 / word_count as f64;
        if comma_ratio > 0.15 {
            signals.push("high_comma_frequency".to_string());
            return 6.0;
        }
    }

    4.0
}

/// Count human informality markers: slang, casual contractions, repeated punctuation.
fn count_informality(text: &str) -> usize {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .collect();
    let mut count = 0;

    // Slang / abbreviations (whole word match)
    for slang in HUMAN_SLANG {
        if words.iter().any(|w| *w == *slang) {
            count += 1;
        }
    }

    // Casual contractions (substring match — "gonna", "kinda", etc.)
    for contraction in CASUAL_CONTRACTIONS {
        if lower.contains(contraction) {
            count += 1;
        }
    }

    // Repeated punctuation (!!, ??, ...)
    if text.contains("!!") || text.contains("??") {
        count += 1;
    }
    if text.contains("...") {
        count += 1;
    }

    count
}

/// Ratio of non-empty lines to sentences — high ratio = one sentence per line (LinkedIn AI).
fn linebreak_ratio(text: &str) -> f64 {
    let lines: Vec<&str> = text
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    if lines.len() < 3 {
        return 0.0; // too few lines to judge
    }

    let sentences: Vec<&str> = text
        .split(|c: char| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.is_empty() {
        return 0.0;
    }

    lines.len() as f64 / sentences.len().max(1) as f64
}

/// Count promotional / motivational patterns common in AI social media posts.
fn count_promotional(text: &str) -> usize {
    let lower = text.to_lowercase();
    PROMOTIONAL_PATTERNS
        .iter()
        .filter(|p| lower.contains(**p))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_text() {
        let text = "lol this is wild!! cant believe what happened today. \
                    so my cat literally knocked over my coffee... again. \
                    third time this week smh. anyone else's cat do this?? \
                    im going crazy here fr";
        let result = analyze(text);
        assert!(result.score <= 4, "Human-like text scored too high: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_ai_text() {
        let text = "In today's world, it's important to note that artificial intelligence \
                    is revolutionizing the way we approach content creation. Furthermore, \
                    the seamless integration of cutting-edge technology enables us to \
                    navigate the complexities of modern communication. Moreover, leveraging \
                    these best practices allows thought leaders to deliver comprehensive \
                    value propositions that drive meaningful engagement.";
        let result = analyze(text);
        assert!(result.score >= 6, "AI-like text scored too low: {} (signals: {:?})", result.score, result.signals);
        assert!(!result.signals.is_empty());
    }

    #[test]
    fn test_linkedin_ai_post() {
        let text = "To Be in the Top 1%, You Must Do What 99% Won't.\n\n\
                    Success is not about working longer hours.\n\
                    It is about thinking and acting differently.\n\n\
                    In business, the most successful entrepreneurs focus on systems.\n\
                    They build processes that scale.\n\
                    They invest in their personal growth.\n\n\
                    Here are the 5 habits that changed my life:\n\
                    1. Wake up at 5 AM\n\
                    2. Read for 30 minutes daily\n\
                    3. Network with purpose\n\
                    4. Track your metrics\n\
                    5. Never stop learning\n\n\
                    Follow for more insights. Repost if you agree.";
        let result = analyze(text);
        assert!(result.score >= 5, "LinkedIn AI post scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_neutral_text() {
        let text = "The company reported quarterly earnings that exceeded analyst expectations. \
                    Revenue grew by twelve percent compared to the same period last year. \
                    The CEO highlighted strong performance in the cloud computing division. \
                    Shares rose three percent in after-hours trading.";
        let result = analyze(text);
        // Neutral/factual text should land in the uncertain range, not confidently human
        assert!(result.score >= 3, "Neutral text scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    // --- Real-world tests from production DB ---
    // These texts were scored by the LLM; the heuristic must not wildly disagree.

    #[test]
    fn test_real_ai_top1percent() {
        // LLM scored 8 — classic LinkedIn motivational AI post
        let text = "To Be in the Top 1%, You Must Do What 99% Won't.\n\n\
                    Success is not about working longer hours.\n\
                    It is about thinking and acting differently.\n\n\
                    In business, many people believe success means:\n\n\
                    • Working 16-hour days\n\
                    • Constantly grinding through exhaustion\n\
                    • Pushing harder when things are not working\n\
                    • Doing more, more, and more\n\n\
                    But that is not what truly separates the top 1%.\n\n\
                    The real difference is this:\n\n\
                    The 1% take action when others hesitate.\n\n\
                    While 99% of people:\n\
                    – Wait for the perfect moment\n\
                    – Overthink every small detail\n\
                    – Make excuses about timing\n\
                    – Stay inside their comfort zone\n\n\
                    The top performers simply start.";
        let result = analyze(text);
        assert!(result.score >= 5, "Real AI post (top 1%) scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_ai_product_promo() {
        // LLM scored 8 — product announcement with CTA
        let text = "New tool is live on GH.\n\n\
                    Chrome/Firefox extension that detects AI-generated content on X, Instagram, and LinkedIn.\n\n\
                    Posts are analyzed using OpenRouter LLM API combined with local heuristic analysis, \
                    with inline score badges.\n\n\
                    Star if you like it pls";
        let result = analyze(text);
        assert!(result.score >= 4, "Real AI promo post scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_ai_giveaway() {
        // LLM scored 7 — promotional giveaway post
        let text = "I'm giving away my entire @openclaw architecture. Behind my $250k/month agency.\n\n\
                    After weeks of building, I've dialled in the exact system that runs my business 24/7.\n\n\
                    What's included:\n\
                    • Memory folder structure (how to organize agent context)\n\
                    • Cron job templates (daily briefs, weekly reports)\n\
                    • Full deployment guide";
        let result = analyze(text);
        assert!(result.score >= 4, "Real AI giveaway post scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_ai_video_myth() {
        // LLM scored 8 — LinkedIn marketing post with "the truth is" pattern
        let text = "\"Video doesn't work on LinkedIn.\"\n\
                    That's the myth. And it's time we put it to rest.\n\
                    The truth?\n\
                    Video is one of the most effective formats on LinkedIn—driving 3x higher engagement, \
                    building brand trust, and even influencing B2B buying decisions.\n\
                    For more best practices, visit the link.";
        let result = analyze(text);
        assert!(result.score >= 4, "Real AI video myth post scored too low: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_human_cold_call() {
        // LLM scored 1 — casual human complaint with slang
        let text = "Someone just cold-called me and asked about SOC 2 support. lol. \
                    Seems @leadiq is also happily sharing my personal phone number.";
        let result = analyze(text);
        assert!(result.score <= 4, "Real human cold-call post scored too high: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_human_bot_trading() {
        // LLM scored 1 — casual technical post, abbreviations, informal
        let text = "AI bot switched 15-min to 5-min btc markets\n\n\
                    $15k today\n\
                    68.7% win rate\n\n\
                    the loop (every 5 minutes):\n\n\
                    ▸ scans btc price across exchanges\n\
                    ▸ detects cex lag vs polymarket odds\n\
                    ▸ enters directional position (up or down)\n\
                    ▸ $1,500-$5,000 per trade\n\
                    ▸ exits on resolution";
        let result = analyze(text);
        assert!(result.score <= 5, "Real human bot-trading post scored too high: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_human_clawdbot() {
        // LLM scored 1 — short casual reaction
        let text = "Can't believe this is basically a 30min clawdbot VPS setup tutorial + custom skill download\n\n\
                    Incredible";
        let result = analyze(text);
        assert!(result.score <= 5, "Real human clawdbot post scored too high: {} (signals: {:?})", result.score, result.signals);
    }

    #[test]
    fn test_real_human_spain_mountain() {
        // LLM scored 1 — personal/descriptive with specific details
        let text = "Someone is selling their mountain in Andalucía (Spain). 280 hectares. For €1.5M.\n\n\
                    That's 692 acres of private wilderness: deer, Ibex, wild boar, partridge - \
                    with a 6-bed cortijo built to a high spec 12 years ago. \
                    Wine cellar accessed through a hatch in the floor, outdoor pool.";
        let result = analyze(text);
        assert!(result.score <= 5, "Real human Spain mountain post scored too high: {} (signals: {:?})", result.score, result.signals);
    }
}
