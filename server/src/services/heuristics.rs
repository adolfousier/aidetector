use std::collections::HashSet;

#[derive(Debug)]
pub struct HeuristicResult {
    pub score: u8,
    pub signals: Vec<String>,
}

const FORMULAIC_PHRASES: &[&str] = &[
    "in today's world",
    "it's important to note",
    "it is important to note",
    "in conclusion",
    "delve into",
    "dive into",
    "let's explore",
    "game changer",
    "game-changer",
    "at the end of the day",
    "leverage",
    "navigate the complexities",
    "in this article",
    "here's the thing",
    "without further ado",
    "it's worth noting",
    "that being said",
    "having said that",
    "comprehensive guide",
    "revolutionize",
    "cutting-edge",
    "seamlessly",
    "furthermore",
    "moreover",
    "in the realm of",
    "paradigm shift",
    "holistic approach",
    "synergy",
    "thought leader",
    "value proposition",
    "best practices",
    "circle back",
    "unpack this",
    "at its core",
    "it goes without saying",
];

pub fn analyze(text: &str) -> HeuristicResult {
    let mut signals = Vec::new();
    let mut score_sum: f64 = 0.0;
    let mut weight_sum: f64 = 0.0;

    // 1. Sentence length variance (AI tends to write uniform sentence lengths)
    let sentence_variance = sentence_length_variance(text);
    let sv_score = if sentence_variance < 5.0 {
        signals.push("uniform_sentence_length".to_string());
        8.0
    } else if sentence_variance < 15.0 {
        signals.push("low_sentence_variance".to_string());
        5.0
    } else {
        2.0
    };
    score_sum += sv_score * 2.0;
    weight_sum += 2.0;

    // 2. Vocabulary diversity (Type-Token Ratio)
    let ttr = type_token_ratio(text);
    let ttr_score = if ttr < 0.4 {
        signals.push("low_vocabulary_diversity".to_string());
        7.0
    } else if ttr < 0.55 {
        signals.push("moderate_vocabulary_diversity".to_string());
        4.0
    } else {
        2.0
    };
    score_sum += ttr_score * 1.5;
    weight_sum += 1.5;

    // 3. Burstiness (AI text tends to have low burstiness — uniform flow)
    let burstiness = compute_burstiness(text);
    let burst_score = if burstiness < 0.3 {
        signals.push("low_burstiness".to_string());
        7.0
    } else if burstiness < 0.5 {
        4.0
    } else {
        2.0
    };
    score_sum += burst_score * 1.5;
    weight_sum += 1.5;

    // 4. Formulaic phrase detection
    let formula_count = count_formulaic_phrases(text);
    let formula_score = if formula_count >= 3 {
        signals.push("formulaic_phrases".to_string());
        9.0
    } else if formula_count >= 1 {
        signals.push("some_formulaic_phrases".to_string());
        5.0
    } else {
        2.0
    };
    score_sum += formula_score * 2.5;
    weight_sum += 2.5;

    // 5. Punctuation patterns (AI uses more consistent punctuation)
    let punct_score = punctuation_analysis(text, &mut signals);
    score_sum += punct_score * 1.0;
    weight_sum += 1.0;

    // 6. Text too short for reliable analysis
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
    let variance = lengths.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / lengths.len() as f64;
    variance
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

fn punctuation_analysis(text: &str, signals: &mut Vec<String>) -> f64 {
    let sentences: Vec<&str> = text
        .split(|c: char| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.len() < 3 {
        return 3.0;
    }

    // Check if almost all sentences end with periods (low variety)
    let total_terminators = text.chars().filter(|c| *c == '.' || *c == '!' || *c == '?').count();
    if total_terminators == 0 {
        return 3.0;
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

    3.0
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
        assert!(result.score <= 5, "Human-like text scored too high: {}", result.score);
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
        assert!(result.score >= 5, "AI-like text scored too low: {}", result.score);
        assert!(!result.signals.is_empty());
    }
}
