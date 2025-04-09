import express from 'express';
import userRoutes from './user';
import oauthProfileRoutes from './oauth_profile';
import userTermsAcceptanceRoutes from './user_terms_acceptance';

const router = express.Router();

router.use('/user', userRoutes);
router.use('/oauthProfile', oauthProfileRoutes);
router.use('/userTermsAcceptance', userTermsAcceptanceRoutes);

export default router;
