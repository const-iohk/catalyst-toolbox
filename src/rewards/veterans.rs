use serde::Deserialize;

pub type VeteranAdvisorId = String;
pub type VeteranAdvisorReward = rust_decimal::Decimal;

pub type VeteranAdvisorRewards = Vec<(VeteranAdvisorId, VeteranAdvisorReward)>;

#[derive(Deserialize)]
pub struct VeteranReviewsCount {
    name: VeteranAdvisorId,
    #[serde(alias = "No. of Reviews")]
    number_of_reviews: usize,
}

pub fn calculate_veteran_advisors_rewards(
    veteran_reviews: &[VeteranReviewsCount],
    base_rewards: VeteranAdvisorReward,
) -> VeteranAdvisorRewards {
    let total_reviews: usize = veteran_reviews.iter().map(|vr| vr.number_of_reviews).sum();

    veteran_reviews
        .iter()
        .map(|vr| {
            (
                vr.name.clone(),
                (VeteranAdvisorReward::from(vr.number_of_reviews)
                    / VeteranAdvisorReward::from(total_reviews))
                    * base_rewards,
            )
        })
        .collect()
}
