import { createApp } from 'https://unpkg.com/petite-vue?module';

//Initialize Stripe
const stripe = Stripe('pk_test_51IqOAQHJovnr3cb7w5jn9rMHREO8tlnHYEZp651nT8Sb8IrftlGVT5oyDDfnHtjPOCQARRjAlg6AP1T48CyfYuVq00K4LMtAvd');

const clientSecret = document.getElementById("payment-form").dataset.secret
const elements = stripe.elements({ clientSecret });
const paymentElement = elements.create('payment');
paymentElement.mount('#payment-element');

createApp({
	$delimiters: ['${', '}'],
	tipAmount: 1,
	customAmount: 1,
	tipAmntErr: false,
	validateAmount(e) {
		const regexp = /^([0-9]{1,3},([0-9]{3},)*[0-9]{3}|[0-9]+)(.[0-9][0-9])?$/
		this.tipAmntErr = !regexp.test(e.target.value);
		this.customAmount = Number.parseFloat(e.target.value);
		if(!this.tipAmntErr) {
			this.updateAmount()
		}
	},
	async updateAmount() {
		let amount = 0
		if(this.tipAmount > 0) {
			amount = this.tipAmount * 100;
		} else if (this.tipAmount == -1) {
			amount = this.customAmount * 100;
		}

		await fetch('/api/tipjar', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				client_secret: clientSecret,
				amount: amount,
			})
		});
	},
	async submit(e) {
		e.preventDefault();
		const { error } = await stripe.confirmPayment({
			elements,
			confirmParams: {
				return_url: "http://localhost:8000/redirect/payment",
			}
		});
		if(error) {
			// TODO: show modal
		}
	}
}).mount()
