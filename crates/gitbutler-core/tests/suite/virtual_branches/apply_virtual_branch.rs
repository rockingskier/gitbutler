use super::*;

#[tokio::test]
async fn deltect_conflict() {
    let Test {
        repository,
        project_id,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(*project_id, &"refs/remotes/origin/master".parse().unwrap())
        .await
        .unwrap();

    let branch1_id = {
        let branch1_id = controller
            .create_virtual_branch(*project_id, &branch::BranchCreateRequest::default())
            .await
            .unwrap();
        fs::write(repository.path().join("file.txt"), "branch one").unwrap();

        branch1_id
    };

    // unapply first vbranch
    let reference_name = controller
        .convert_to_real_branch(*project_id, branch1_id, Default::default())
        .await
        .unwrap();

    let refname = git::Refname::from_str(reference_name.as_str()).unwrap();

    {
        // create another vbranch that conflicts with the first one
        controller
            .create_virtual_branch(*project_id, &branch::BranchCreateRequest::default())
            .await
            .unwrap();
        fs::write(repository.path().join("file.txt"), "branch two").unwrap();
    }

    {
        assert!(matches!(
            controller
                .create_virtual_branch_from_branch(*project_id, &refname)
                .await
                .unwrap_err()
                .downcast_ref(),
            Some(Marker::ProjectConflict)
        ));
    }
}

#[tokio::test]
async fn rebase_commit() {
    let Test {
        repository,
        project_id,
        controller,
        ..
    } = &Test::default();

    // make sure we have an undiscovered commit in the remote branch
    {
        fs::write(repository.path().join("file.txt"), "one").unwrap();
        fs::write(repository.path().join("another_file.txt"), "").unwrap();
        let first_commit_oid = repository.commit_all("first");
        fs::write(repository.path().join("file.txt"), "two").unwrap();
        repository.commit_all("second");
        repository.push();
        repository.reset_hard(Some(first_commit_oid));
    }

    controller
        .set_base_branch(*project_id, &"refs/remotes/origin/master".parse().unwrap())
        .await
        .unwrap();

    let mut branch1_id = {
        // create a branch with some commited work
        let branch1_id = controller
            .create_virtual_branch(*project_id, &branch::BranchCreateRequest::default())
            .await
            .unwrap();
        fs::write(repository.path().join("another_file.txt"), "virtual").unwrap();

        controller
            .create_commit(*project_id, branch1_id, "virtual commit", None, false)
            .await
            .unwrap();

        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert!(branches[0].active);
        assert_eq!(branches[0].files.len(), 0);
        assert_eq!(branches[0].commits.len(), 1);

        branch1_id
    };

    let reference_name = {
        // unapply first vbranch
        let reference_name = controller
            .convert_to_real_branch(*project_id, branch1_id, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs::read_to_string(repository.path().join("another_file.txt")).unwrap(),
            ""
        );
        assert_eq!(
            fs::read_to_string(repository.path().join("file.txt")).unwrap(),
            "one"
        );

        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 0);
        assert_eq!(branches[0].commits.len(), 1);
        assert!(!branches[0].active);

        reference_name
    };

    let refname = git::Refname::from_str(reference_name.as_str()).unwrap();

    {
        // fetch remote
        controller.update_base_branch(*project_id).await.unwrap();

        // branch is stil unapplied
        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 0);
        assert_eq!(branches[0].commits.len(), 1);
        assert!(!branches[0].active);
        assert!(!branches[0].conflicted);

        assert_eq!(
            fs::read_to_string(repository.path().join("another_file.txt")).unwrap(),
            ""
        );
        assert_eq!(
            fs::read_to_string(repository.path().join("file.txt")).unwrap(),
            "two"
        );
    }

    {
        // apply first vbranch again
        branch1_id = controller
            .create_virtual_branch_from_branch(*project_id, &refname)
            .await
            .unwrap()
            .0;

        // it should be rebased
        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 0);
        assert_eq!(branches[0].commits.len(), 1);
        assert!(branches[0].active);
        assert!(!branches[0].conflicted);

        assert_eq!(
            fs::read_to_string(repository.path().join("another_file.txt")).unwrap(),
            "virtual"
        );

        assert_eq!(
            fs::read_to_string(repository.path().join("file.txt")).unwrap(),
            "two"
        );
    }
}

#[tokio::test]
async fn rebase_work() {
    let Test {
        repository,
        project_id,
        controller,
        ..
    } = &Test::default();

    // make sure we have an undiscovered commit in the remote branch
    {
        let first_commit_oid = repository.commit_all("first");
        fs::write(repository.path().join("file.txt"), "").unwrap();
        repository.commit_all("second");
        repository.push();
        repository.reset_hard(Some(first_commit_oid));
    }

    controller
        .set_base_branch(*project_id, &"refs/remotes/origin/master".parse().unwrap())
        .await
        .unwrap();

    let mut branch1_id = {
        // make a branch with some work
        let branch1_id = controller
            .create_virtual_branch(*project_id, &branch::BranchCreateRequest::default())
            .await
            .unwrap();
        fs::write(repository.path().join("another_file.txt"), "").unwrap();

        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert!(branches[0].active);
        assert_eq!(branches[0].files.len(), 1);
        assert_eq!(branches[0].commits.len(), 0);

        branch1_id
    };

    let reference_name = {
        // unapply first vbranch
        let reference_name = controller
            .convert_to_real_branch(*project_id, branch1_id, Default::default())
            .await
            .unwrap();

        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 1);
        assert_eq!(branches[0].commits.len(), 0);
        assert!(!branches[0].active);

        assert!(!repository.path().join("another_file.txt").exists());
        assert!(!repository.path().join("file.txt").exists());

        reference_name
    };

    let refname = git::Refname::from_str(reference_name.as_str()).unwrap();

    {
        // fetch remote
        controller.update_base_branch(*project_id).await.unwrap();

        // first branch is stil unapplied
        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 1);
        assert_eq!(branches[0].commits.len(), 0);
        assert!(!branches[0].active);
        assert!(!branches[0].conflicted);

        assert!(!repository.path().join("another_file.txt").exists());
        assert!(repository.path().join("file.txt").exists());
    }

    {
        // apply first vbranch again
        branch1_id = controller
            .create_virtual_branch_from_branch(*project_id, &refname)
            .await
            .unwrap()
            .0;

        // workdir should be rebased, and work should be restored
        let (branches, _) = controller.list_virtual_branches(*project_id).await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].files.len(), 1);
        assert_eq!(branches[0].commits.len(), 0);
        assert!(branches[0].active);
        assert!(!branches[0].conflicted);

        assert!(repository.path().join("another_file.txt").exists());
        assert!(repository.path().join("file.txt").exists());
    }
}
