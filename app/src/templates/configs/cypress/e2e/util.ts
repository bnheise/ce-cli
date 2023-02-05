// This helper function logs you in to carry out tests as a specific user.
export const login = (username: string, password: string) => {
	cy.get(
		'#p_p_id_com_liferay_product_navigation_user_personal_bar_web_portlet_ProductNavigationUserPersonalBarPortlet_'
	).click();

	cy.get('#_com_liferay_login_web_portlet_LoginPortlet_login')
		.clear()
		.type(username);

	cy.get('#_com_liferay_login_web_portlet_LoginPortlet_password').type(
		password
	);

	cy.get(
		'[id^=_com_liferay_login_web_portlet_LoginPortlet] button[type="submit"]'
	).click();
};
