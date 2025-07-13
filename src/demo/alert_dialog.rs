use leptos::prelude::*;
use crate::components::v1::{
    alert_dialog::{
        AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
        AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
        AlertDialogTrigger,
    },
    button::{Button, ButtonVariant},
};

#[component]
pub fn AlertDialogDemo() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <Button variant=Signal::from(ButtonVariant::Outline)>"Show Dialog"</Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                    <AlertDialogAction>"Continue"</AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
