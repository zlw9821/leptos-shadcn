use leptos::prelude::*;

#[component]
pub fn Table() -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="table">
                <caption>A list of your recent invoices.</caption>
                <thead>
                    <tr>
                        <th>Invoice</th>
                        <th>Status</th>
                        <th>Method</th>
                        <th>Amount</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="font-medium">INV001</td>
                        <td>Paid</td>
                        <td>Credit Card</td>
                        <td class="text-right">$250.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV002</td>
                        <td>Pending</td>
                        <td>PayPal</td>
                        <td class="text-right">$150.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV003</td>
                        <td>Unpaid</td>
                        <td>Bank Transfer</td>
                        <td class="text-right">$350.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV004</td>
                        <td>Paid</td>
                        <td>Paypal</td>
                        <td class="text-right">$450.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV005</td>
                        <td>Paid</td>
                        <td>Credit Card</td>
                        <td class="text-right">$550.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV006</td>
                        <td>Pending</td>
                        <td>Bank Transfer</td>
                        <td class="text-right">$200.00</td>
                    </tr>
                    <tr>
                        <td class="font-medium">INV007</td>
                        <td>Unpaid</td>
                        <td>Credit Card</td>
                        <td class="text-right">$300.00</td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td colspan="3">Total</td>
                        <td class="text-right">$2,500.00</td>
                    </tr>
                </tfoot>
            </table>
        </div>
    }
}