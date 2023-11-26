table! {
    /// Representation of the `fcp_concern` table.
    ///
    /// (Automatically generated by Diesel.)
    fcp_concern (id) {
        /// The `id` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_proposal` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_proposal -> Int4,
        /// The `fk_initiator` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiator -> Int4,
        /// The `fk_resolved_comment` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_resolved_comment -> Nullable<Int4>,
        /// The `name` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `fk_initiating_comment` column of the `fcp_concern` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiating_comment -> Int4,
    }
}

table! {
    /// Representation of the `fcp_proposal` table.
    ///
    /// (Automatically generated by Diesel.)
    fcp_proposal (id) {
        /// The `id` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_issue` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_issue -> Int4,
        /// The `fk_initiator` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiator -> Int4,
        /// The `fk_initiating_comment` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiating_comment -> Int4,
        /// The `disposition` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        disposition -> Varchar,
        /// The `fk_bot_tracking_comment` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_bot_tracking_comment -> Int4,
        /// The `fcp_start` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        fcp_start -> Nullable<Timestamp>,
        /// The `fcp_closed` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        fcp_closed -> Bool,
        /// The `teams` column of the `fcp_proposal` table.
        ///
        /// Its SQL type is `Nullable<Array<Text>>`.
        ///
        /// (Automatically generated by Diesel.)
        teams -> Nullable<Array<Text>>,
    }
}

table! {
    /// Representation of the `fcp_review_request` table.
    ///
    /// (Automatically generated by Diesel.)
    fcp_review_request (id) {
        /// The `id` column of the `fcp_review_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_proposal` column of the `fcp_review_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_proposal -> Int4,
        /// The `fk_reviewer` column of the `fcp_review_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_reviewer -> Int4,
        /// The `reviewed` column of the `fcp_review_request` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        reviewed -> Bool,
    }
}

table! {
    /// Representation of the `githubsync` table.
    ///
    /// (Automatically generated by Diesel.)
    githubsync (id) {
        /// The `id` column of the `githubsync` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `successful` column of the `githubsync` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        successful -> Bool,
        /// The `ran_at` column of the `githubsync` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        ran_at -> Timestamp,
        /// The `message` column of the `githubsync` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        message -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `githubuser` table.
    ///
    /// (Automatically generated by Diesel.)
    githubuser (id) {
        /// The `id` column of the `githubuser` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `login` column of the `githubuser` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        login -> Varchar,
    }
}

table! {
    /// Representation of the `issue` table.
    ///
    /// (Automatically generated by Diesel.)
    issue (id) {
        /// The `id` column of the `issue` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `number` column of the `issue` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        number -> Int4,
        /// The `fk_milestone` column of the `issue` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_milestone -> Nullable<Int4>,
        /// The `fk_user` column of the `issue` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_user -> Int4,
        /// The `fk_assignee` column of the `issue` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_assignee -> Nullable<Int4>,
        /// The `open` column of the `issue` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        open -> Bool,
        /// The `is_pull_request` column of the `issue` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_pull_request -> Bool,
        /// The `title` column of the `issue` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Varchar,
        /// The `body` column of the `issue` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Varchar,
        /// The `locked` column of the `issue` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        locked -> Bool,
        /// The `closed_at` column of the `issue` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        closed_at -> Nullable<Timestamp>,
        /// The `created_at` column of the `issue` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `issue` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `labels` column of the `issue` table.
        ///
        /// Its SQL type is `Array<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        labels -> Array<Text>,
        /// The `repository` column of the `issue` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Varchar,
    }
}

table! {
    /// Representation of the `issuecomment` table.
    ///
    /// (Automatically generated by Diesel.)
    issuecomment (id) {
        /// The `id` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_issue` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_issue -> Int4,
        /// The `fk_user` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_user -> Int4,
        /// The `body` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Varchar,
        /// The `created_at` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `repository` column of the `issuecomment` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Varchar,
    }
}

table! {
    /// Representation of the `memberships` table.
    ///
    /// (Automatically generated by Diesel.)
    memberships (id) {
        /// The `id` column of the `memberships` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_member` column of the `memberships` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_member -> Int4,
        /// The `fk_team` column of the `memberships` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_team -> Int4,
    }
}

table! {
    /// Representation of the `milestone` table.
    ///
    /// (Automatically generated by Diesel.)
    milestone (id) {
        /// The `id` column of the `milestone` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `number` column of the `milestone` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        number -> Int4,
        /// The `open` column of the `milestone` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        open -> Bool,
        /// The `title` column of the `milestone` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Varchar,
        /// The `description` column of the `milestone` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Varchar>,
        /// The `fk_creator` column of the `milestone` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_creator -> Int4,
        /// The `open_issues` column of the `milestone` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        open_issues -> Int4,
        /// The `closed_issues` column of the `milestone` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        closed_issues -> Int4,
        /// The `created_at` column of the `milestone` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `milestone` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `closed_at` column of the `milestone` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        closed_at -> Nullable<Timestamp>,
        /// The `due_on` column of the `milestone` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        due_on -> Nullable<Timestamp>,
        /// The `repository` column of the `milestone` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Varchar,
    }
}

table! {
    /// Representation of the `poll` table.
    ///
    /// (Automatically generated by Diesel.)
    poll (id) {
        /// The `id` column of the `poll` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_issue` column of the `poll` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_issue -> Int4,
        /// The `fk_initiator` column of the `poll` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiator -> Int4,
        /// The `fk_initiating_comment` column of the `poll` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiating_comment -> Int4,
        /// The `fk_bot_tracking_comment` column of the `poll` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_bot_tracking_comment -> Int4,
        /// The `poll_question` column of the `poll` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        poll_question -> Varchar,
        /// The `poll_created_at` column of the `poll` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        poll_created_at -> Timestamp,
        /// The `poll_closed` column of the `poll` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        poll_closed -> Bool,
        /// The `poll_teams` column of the `poll` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        poll_teams -> Varchar,
    }
}

table! {
    /// Representation of the `poll_response_request` table.
    ///
    /// (Automatically generated by Diesel.)
    poll_response_request (id) {
        /// The `id` column of the `poll_response_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_poll` column of the `poll_response_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_poll -> Int4,
        /// The `fk_respondent` column of the `poll_response_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_respondent -> Int4,
        /// The `responded` column of the `poll_response_request` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        responded -> Bool,
    }
}

table! {
    /// Representation of the `pullrequest` table.
    ///
    /// (Automatically generated by Diesel.)
    pullrequest (id) {
        /// The `id` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `number` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        number -> Int4,
        /// The `state` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        state -> Varchar,
        /// The `title` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Varchar,
        /// The `body` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Nullable<Varchar>,
        /// The `fk_assignee` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_assignee -> Nullable<Int4>,
        /// The `fk_milestone` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_milestone -> Nullable<Int4>,
        /// The `locked` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        locked -> Bool,
        /// The `created_at` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `closed_at` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        closed_at -> Nullable<Timestamp>,
        /// The `merged_at` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        merged_at -> Nullable<Timestamp>,
        /// The `commits` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        commits -> Int4,
        /// The `additions` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        additions -> Int4,
        /// The `deletions` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        deletions -> Int4,
        /// The `changed_files` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        changed_files -> Int4,
        /// The `repository` column of the `pullrequest` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Varchar,
    }
}

table! {
    /// Representation of the `rfc_feedback_request` table.
    ///
    /// (Automatically generated by Diesel.)
    rfc_feedback_request (id) {
        /// The `id` column of the `rfc_feedback_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `fk_initiator` column of the `rfc_feedback_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_initiator -> Int4,
        /// The `fk_requested` column of the `rfc_feedback_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_requested -> Int4,
        /// The `fk_issue` column of the `rfc_feedback_request` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        fk_issue -> Int4,
        /// The `fk_feedback_comment` column of the `rfc_feedback_request` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        fk_feedback_comment -> Nullable<Int4>,
    }
}

table! {
    /// Representation of the `teams` table.
    ///
    /// (Automatically generated by Diesel.)
    teams (id) {
        /// The `id` column of the `teams` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `teams` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `ping` column of the `teams` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        ping -> Varchar,
        /// The `label` column of the `teams` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        label -> Varchar,
    }
}

joinable!(fcp_concern -> fcp_proposal (fk_proposal));
joinable!(fcp_concern -> githubuser (fk_initiator));
joinable!(fcp_proposal -> githubuser (fk_initiator));
joinable!(fcp_proposal -> issue (fk_issue));
joinable!(fcp_review_request -> fcp_proposal (fk_proposal));
joinable!(fcp_review_request -> githubuser (fk_reviewer));
joinable!(issue -> milestone (fk_milestone));
joinable!(issuecomment -> githubuser (fk_user));
joinable!(issuecomment -> issue (fk_issue));
joinable!(memberships -> githubuser (fk_member));
joinable!(memberships -> teams (fk_team));
joinable!(milestone -> githubuser (fk_creator));
joinable!(poll -> githubuser (fk_initiator));
joinable!(poll -> issue (fk_issue));
joinable!(poll_response_request -> githubuser (fk_respondent));
joinable!(poll_response_request -> poll (fk_poll));
joinable!(pullrequest -> githubuser (fk_assignee));
joinable!(pullrequest -> milestone (fk_milestone));
joinable!(rfc_feedback_request -> issue (fk_issue));
joinable!(rfc_feedback_request -> issuecomment (fk_feedback_comment));

allow_tables_to_appear_in_same_query!(
    fcp_concern,
    fcp_proposal,
    fcp_review_request,
    githubsync,
    githubuser,
    issue,
    issuecomment,
    memberships,
    milestone,
    poll,
    poll_response_request,
    pullrequest,
    rfc_feedback_request,
    teams,
);
